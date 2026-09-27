use crate::transport::sync::connection::run_connection;
use crate::transport::sync::reader::FixMessageReader;
use crate::transport::sync::responder::TcpResponder;
use crate::session::{Session, SessionId};
use getset::Getters;
use log::{info, warn};
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// How long to sleep between session-window checks while *outside* the session
/// window. Deliberately shorter than the reconnect interval so the initiator
/// starts dialing promptly once the window opens, rather than up to a full
/// (possibly long) reconnect interval late.
const OUT_OF_SESSION_POLL_SECS: u64 = 5;

/// Dials out to a single counterparty for one already-known session (the mirror of
/// `IoAcceptor`). Unlike the acceptor, the initiator chose the counterparty, so it
/// knows its `SessionId` up front from config — there is no first-message lookup.
///
/// Holds only that one session's `Arc<Mutex<Session>>` (a clone of the entry in the
/// shared `SessionMap`), not the whole map: the map is only needed to *identify* an
/// inbound connection, which the initiator never has to do. The session still lives in
/// the map so the timer thread ticks it — and it is the timer's `next_tick` that
/// actually sends the Logon once the responder below is wired.
#[derive(Getters)]
pub struct IoInitiator {
    #[get = "pub"]
    session_id: SessionId,
    session: Arc<Mutex<Session>>,
    #[get = "pub"]
    connect_addr: SocketAddr,
    stop: Arc<AtomicBool>,
}

impl IoInitiator {
    pub fn new(
        session_id: SessionId,
        session: Arc<Mutex<Session>>,
        connect_addr: SocketAddr,
    ) -> IoInitiator {
        Self {
            session_id,
            session,
            connect_addr,
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    fn interruptible_sleep(&self, total: Duration) {
        let step = Duration::from_millis(1000);
        let mut slept = Duration::ZERO;
        while slept < total {
            if self.stop.load(Ordering::Relaxed) {
                return;
            }
            thread::sleep(step);
            slept += step;
        }
    }
    /// Connects to the counterparty and runs the session until the peer disconnects.
    ///
    /// `TcpStream::connect` opens the outbound socket, then it is split via `try_clone`
    /// — one handle goes to the `TcpResponder` (writes), the other to the
    /// `FixMessageReader` (reads). Setting the responder is all this does to start the
    /// handshake: the timer's `next_tick` sees `is_initiator && !logon_sent && responder
    /// present` and sends the Logon on its next pass. From there the shared pump loop
    /// reads and dispatches (including the Logon response) until the connection drops.
    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let reconnect_interval = self.session.lock().unwrap().reconnect_interval().unwrap_or(30);
        let connection_loop = || {
            let stream = TcpStream::connect(self.connect_addr)?;
            let write_clone = stream.try_clone()?;
            self.session
                .lock()
                .unwrap()
                .set_responder(Some(Box::new(TcpResponder::new(write_clone))));
            let reader = FixMessageReader::new(stream);
            // Log under the session's own-perspective id, matching the acceptor.
            run_connection(self.session.clone(), self.session_id.id(), reader, self.connect_addr)
        };
        loop {
            // Stop request (set from another thread via `stop`) ends the reconnect loop
            // cleanly. interruptible_sleep wakes on the same flag, so a stop during a
            // wait falls through to here on the next pass and returns. Note: this only
            // catches the loop between connections — a stop while parked in
            // run_connection's blocking read needs the socket to close first.
            if self.stop.load(Ordering::Relaxed) {
                return Ok(());
            }
            // Only dial inside the configured session window. Outside it, poll on a
            // short interval (see OUT_OF_SESSION_POLL_SECS) instead of dialing.
            let is_session_time = self.session.lock().unwrap().schedule().is_session_time();
            if !is_session_time {
                self.interruptible_sleep(Duration::from_secs(OUT_OF_SESSION_POLL_SECS));
                continue;
            }

            // In session: dial and run until the connection ends. connection_loop()
            // returns Ok on a clean peer disconnect (EOF/Logout) and Err on a failed
            // dial or a session/IO error. Either way we wait a reconnect interval and
            // re-dial — the outcome only changes the log level, so an operator can see
            // *why* the session dropped and that a retry is coming.
            match connection_loop() {
                Ok(()) => info!(
                    "session {} disconnected from {}; reconnecting in {}s",
                    self.session_id.id(),
                    self.connect_addr,
                    reconnect_interval
                ),
                Err(e) => warn!(
                    "connection to {} failed: {}; retrying in {}s",
                    self.connect_addr, e, reconnect_interval
                ),
            }
            self.interruptible_sleep(Duration::from_secs(reconnect_interval as u64));
        }
    }
}

#[cfg(test)]
mod initiator_tests {
    use super::*;
    use crate::application::DefaultApplication;
    use crate::session::SessionProperties;
    use std::io::Read;
    use std::net::{Shutdown, TcpListener};
    use std::time::{Duration, Instant};

    // Builds an active initiator session the real way (config -> to_session), so
    // is_active/is_initiator/non-stop-schedule are all set exactly as at runtime.
    // The connect port here is a placeholder — the test passes the live listener
    // address into IoInitiator::new directly. `reconnect_interval` (seconds) tunes
    // how fast the reconnect loop re-dials; None falls back to the engine default
    // (30s), which is fine for tests that never trigger a reconnect.
    fn make_initiator_session(reconnect_interval: Option<u16>) -> (SessionId, Arc<Mutex<Session>>) {
        let reconnect_line = match reconnect_interval {
            Some(secs) => format!("reconnect_interval = {}", secs),
            None => String::new(),
        };
        let cfg = format!(
            r#"
            [Default]
            begin_string = "FIX.4.3"
            data_dictionary = "resources/FIX43.xml"

            [[Session]]
            connection_type = "initiator"
            sender_comp_id = "INITIATOR"
            target_comp_id = "ACCEPTOR"
            socket_connect_host = "127.0.0.1"
            socket_connect_port = 1
            heartbeat_interval = 30
            {}
        "#,
            reconnect_line
        );
        let props = SessionProperties::from_str(&cfg).unwrap();
        let (id, config) = props.sessions().iter().next().unwrap();
        let session = config.to_session(Box::new(DefaultApplication::new()));
        (id.clone(), Arc::new(Mutex::new(session)))
    }

    // Accepts one connection within a bounded budget instead of blocking forever, so
    // a broken (never-)reconnect fails the test with a clear message rather than
    // hanging it. Returns a blocking stream (read timeouts need blocking mode).
    fn accept_within(listener: &TcpListener, budget: Duration) -> TcpStream {
        listener.set_nonblocking(true).unwrap();
        let deadline = Instant::now() + budget;
        while Instant::now() < deadline {
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(false).unwrap();
                    return stream;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(20));
                }
                Err(e) => panic!("accept failed: {}", e),
            }
        }
        panic!("no connection accepted within {:?} (initiator did not dial)", budget);
    }

    // Plays the session timer (next_tick) until a full FIX message lands on `peer`
    // (checksum tag `10=` seen) or the tick budget runs out, returning whatever was
    // read. The Logon is only emitted once next_tick sees the responder wired, so we
    // tick-then-read in a loop; each miss just retries on the 100ms read timeout, so
    // there are no fixed sleeps to be flaky. Reused for the first connection AND the
    // post-reconnect one.
    fn drive_until_message(peer: &mut TcpStream, session: &Arc<Mutex<Session>>) -> String {
        peer.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        let mut received = String::new();
        let mut buf = [0u8; 1024];
        for _ in 0..100 {
            session.lock().unwrap().next_tick();
            match peer.read(&mut buf) {
                Ok(0) => break, // peer closed
                Ok(n) => {
                    received.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if received.contains("10=") {
                        break; // checksum tag => full message received
                    }
                }
                Err(_) => continue, // read timeout: nothing yet, tick again
            }
        }
        received
    }

    // Signals the initiator's stop flag, closes the peer so the blocking read in
    // run_connection returns, and joins the thread. The order matters: stop first so
    // the loop returns instead of re-dialing once the read unblocks, then shutdown to
    // unblock it. Without the shutdown, join() would hang on the parked read.
    fn stop_and_join(stop: &Arc<AtomicBool>, peer: &TcpStream, handle: thread::JoinHandle<()>) {
        stop.store(true, Ordering::Relaxed);
        peer.shutdown(Shutdown::Both).unwrap();
        handle.join().unwrap();
    }

    // End-to-end over a real loopback socket: the initiator dials out, wires the
    // responder, and the (test-driven) timer sends the Logon.
    //
    // start() is an infinite reconnect loop, so we grab its stop handle before it moves
    // into the thread, assert on the wire output, then stop + join for a clean teardown
    // (no orphaned threads).
    #[test]
    fn test_initiator_connects_wires_responder_and_sends_logon() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (id, session_arc) = make_initiator_session(None);
        let initiator = IoInitiator::new(id, Arc::clone(&session_arc), addr);
        let stop = Arc::clone(&initiator.stop);
        let handle = thread::spawn(move || {
            let _ = initiator.start();
        });

        let mut peer = accept_within(&listener, Duration::from_secs(10));
        let received = drive_until_message(&mut peer, &session_arc);

        let pipe = |s: &str| s.replace('\x01', "|");
        assert!(
            received.contains("\x0135=A\x01"),
            "expected Logon (35=A), got: {}",
            pipe(&received)
        );
        assert!(
            received.contains("\x0149=INITIATOR\x01"),
            "Logon SenderCompID wrong: {}",
            pipe(&received)
        );
        assert!(
            received.contains("\x0156=ACCEPTOR\x01"),
            "Logon TargetCompID wrong: {}",
            pipe(&received)
        );

        stop_and_join(&stop, &peer, handle);
    }

    // The 8.5 reconnect loop: after the peer drops, the initiator waits its reconnect
    // interval and re-dials, and the fresh connection completes a NEW Logon. This is
    // the task's "done when" — killing the peer socket makes the initiator retry and
    // re-logon when the peer returns.
    //
    // Covers the core positive class (clean drop -> reconnect -> re-logon). A short
    // reconnect_interval (1s) keeps it fast; the non-stop schedule means the loop
    // always dials. The single listener accepts TWICE: the second accept only returns
    // once the initiator has re-dialed after the drop, so it IS the reconnect.
    //
    // Residual gaps left deliberately (noted per fix-test-design): connect-fails-then-
    // succeeds retry (needs a late-bound listener), out-of-session-time gating (needs a
    // windowed schedule fixture), and that the reconnect *interval* is actually honored
    // as a duration (timing-sensitive/flaky). All separate classes from this one.
    #[test]
    fn test_initiator_reconnects_and_relogons_after_peer_drop() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (id, session_arc) = make_initiator_session(Some(1));
        let initiator = IoInitiator::new(id, Arc::clone(&session_arc), addr);
        let stop = Arc::clone(&initiator.stop);
        let handle = thread::spawn(move || {
            let _ = initiator.start();
        });

        // First connection: Logon goes out, then we drop the peer to force a reconnect.
        let mut peer1 = accept_within(&listener, Duration::from_secs(10));
        let first = drive_until_message(&mut peer1, &session_arc);
        assert!(
            first.contains("\x0135=A\x01"),
            "expected first Logon (35=A), got: {}",
            first.replace('\x01', "|")
        );
        peer1.shutdown(Shutdown::Both).unwrap();

        // The read loop hits EOF -> disconnect() clears the responder and resets the
        // logon flags -> the reconnect loop waits ~1s and re-dials, which this second
        // accept picks up. (A never-reconnect regression trips accept_within's budget.)
        let mut peer2 = accept_within(&listener, Duration::from_secs(10));

        // Second connection: next_tick sees is_initiator && !logon_sent && responder set
        // again, so a fresh Logon must be emitted on the NEW socket. If the flags didn't
        // reset on disconnect, no Logon comes and this assertion fails.
        let second = drive_until_message(&mut peer2, &session_arc);
        let pipe = |s: &str| s.replace('\x01', "|");
        assert!(
            second.contains("\x0135=A\x01"),
            "expected Logon on reconnect (35=A), got: {}",
            pipe(&second)
        );
        assert!(
            second.contains("\x0149=INITIATOR\x01"),
            "reconnect Logon SenderCompID wrong: {}",
            pipe(&second)
        );
        assert!(
            second.contains("\x0156=ACCEPTOR\x01"),
            "reconnect Logon TargetCompID wrong: {}",
            pipe(&second)
        );

        stop_and_join(&stop, &peer2, handle);
    }
}
