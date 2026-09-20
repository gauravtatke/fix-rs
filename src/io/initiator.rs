use crate::io::connection::run_connection;
use crate::io::fix_message_reader::FixMessageReader;
use crate::io::tcp_responder::TcpResponder;
use crate::session::{Session, SessionId};
use getset::Getters;
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

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
        let stream = TcpStream::connect(self.connect_addr)?;
        let write_clone = stream.try_clone()?;
        self.session.lock().unwrap().set_responder(Some(Box::new(TcpResponder::new(write_clone))));
        let reader = FixMessageReader::new(stream);
        // Log under the session's own-perspective id, matching the acceptor.
        run_connection(self.session.clone(), self.session_id.id(), reader, self.connect_addr)
    }
}

#[cfg(test)]
mod initiator_tests {
    use super::*;
    use crate::application::DefaultApplication;
    use crate::session::SessionProperties;
    use std::io::Read;
    use std::net::{Shutdown, TcpListener};
    use std::time::Duration;

    // Builds an active initiator session the real way (config -> to_session), so
    // is_active/is_initiator/non-stop-schedule are all set exactly as at runtime.
    // The connect port here is a placeholder — the test passes the live listener
    // address into IoInitiator::new directly.
    fn make_initiator_session() -> (SessionId, Arc<Mutex<Session>>) {
        let cfg = r#"
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
        "#;
        let props = SessionProperties::from_str(cfg).unwrap();
        let (id, config) = props.sessions().iter().next().unwrap();
        let session = config.to_session(Box::new(DefaultApplication::new()));
        (id.clone(), Arc::new(Mutex::new(session)))
    }

    // End-to-end over a real loopback socket: the initiator dials out, wires the
    // responder, the (test-driven) timer sends the Logon, and closing the peer
    // tears the session down so start() returns.
    #[test]
    fn test_initiator_connects_wires_responder_and_sends_logon() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (id, session_arc) = make_initiator_session();
        let initiator = IoInitiator::new(id, Arc::clone(&session_arc), addr);

        // start() dials `addr`, sets the responder, then blocks in run_connection
        // reading responses — so it must run on its own thread. Collapse the result
        // to a bool inside the thread: Box<dyn Error> isn't Send, so it can't cross
        // the join boundary.
        let handle = std::thread::spawn(move || initiator.start().is_ok());

        // Stand-in acceptor accepts the outbound connection.
        let (mut peer, _) = listener.accept().unwrap();
        peer.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

        // The Logon is emitted by next_tick once the responder is wired. We play the
        // timer here: tick until the Logon lands. Each miss (responder not wired yet)
        // just retries on the read timeout, so there are no fixed sleeps to be flaky.
        let mut received = String::new();
        let mut buf = [0u8; 1024];
        for _ in 0..100 {
            session_arc.lock().unwrap().next_tick();
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

        // Closing the peer drives the initiator's read loop to EOF → run_connection
        // tears down and start() returns Ok.
        peer.shutdown(Shutdown::Both).unwrap();
        let start_ok = handle.join().unwrap();
        assert!(start_ok, "start() should return Ok after a clean disconnect");
    }
}
