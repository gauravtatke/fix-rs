use crate::io::fix_message_reader::FixMessageReader;
use crate::io::tcp_responder::TcpResponder;
use crate::message::{self, Message};
use crate::network::SessionMap;
use crate::session::Session;
use log::{info, warn};
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::thread;

/// Listens on a single bind address and spawns one thread per inbound TCP connection.
/// Multiple sessions can share the same bind address — the first message (Logon) on
/// each connection determines which session owns it.
pub struct IoAcceptor {
    bind_addr: SocketAddr,
    session_map: SessionMap,
}

impl IoAcceptor {
    pub fn new(addr: SocketAddr, session_map: SessionMap) -> IoAcceptor {
        Self {
            bind_addr: addr,
            session_map,
        }
    }

    /// Binds a `TcpListener` and blocks forever accepting connections.
    /// Each accepted connection is handed to `handle_connection` on its own thread.
    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.bind_addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let session_map = self.session_map.clone();
            let _ = thread::spawn(move || {
                let _ = handle_connection(stream, session_map);
            });
        }
        Ok(())
    }
}

/// Per-connection lifecycle: identify the session from the first (Logon) message,
/// wire the responder, then loop reading and dispatching until the peer disconnects.
///
/// `stream` is split via `try_clone` — the original handle goes to `TcpResponder`
/// (writes), the clone goes to `FixMessageReader` (reads). The session mutex is
/// locked only for the dispatch of each message, never across a blocking read.
fn handle_connection(stream: TcpStream, session_map: SessionMap) -> Result<(), Box<dyn Error>> {
    let peer_addr = stream.peer_addr()?;
    info!("Connection accepted from {}", peer_addr);
    let read_clone = stream.try_clone()?;
    let mut fix_msg_reader = FixMessageReader::new(read_clone);

    // First message must be Logon — used to identify which session this connection belongs to.
    let first_msg_str = fix_msg_reader.read_message()?;
    let reverse_id = message::reverse_session_id_from_raw(&first_msg_str);
    let Some(s_arc) = session_map.get(&reverse_id) else {
        info!("No session found for {}, dropping connection from {}", reverse_id, peer_addr);
        return Ok(());
    };
    info!("{} incoming: {}", reverse_id, wire_display(&first_msg_str));
    // Establish the session from the first (Logon) message under a single lock.
    // On Break the first message failed (malformed, or a fatal/rejected logon) and
    // the session is already disconnected — close out (reusing the same guard, so
    // no re-lock) and return without entering the read loop. On the other path the
    // guard drops at the end of this block, before the loop starts locking per-read.
    {
        let mut session = s_arc.lock().unwrap();
        if establish_session(&mut session, stream, &first_msg_str).is_break() {
            close_connection(&mut session, peer_addr);
            return Ok(());
        }
    }

    while let Ok(msg_str) = fix_msg_reader.read_message() {
        info!("{} incoming: {}", reverse_id, wire_display(&msg_str));
        let mut session = s_arc.lock().unwrap();
        // Break => session already torn down inside next_message; stop reading.
        // The tail close_connection logs the close and disconnects (idempotent).
        if process_inbound_msg(&mut session, &msg_str).is_break() {
            break;
        }
    }
    let mut session = s_arc.lock().unwrap();
    close_connection(&mut session, peer_addr);
    Ok(())
}

/// Establishes the session from the first (Logon) message: wires the responder,
/// then parses and dispatches it. Returns the read-loop signal — Continue to
/// proceed to the loop, or Break if the session is already torn down (a malformed
/// first message, or a fatal / rejected logon). Runs under the caller's lock
/// (takes an already-locked `&mut Session`, so it never locks itself).
fn establish_session(session: &mut Session, stream: TcpStream, raw: &str) -> ControlFlow<()> {
    session.set_responder(Some(Box::new(TcpResponder::new(stream))));
    match Message::from_str(raw, session.data_dict()) {
        Ok(mut first_msg) => session.next_message(&mut first_msg),
        Err(reason) => {
            // A malformed first message can't establish a session — there is no
            // logon to reject. Log the reason (our only record, since this is no
            // longer propagated as an error) and drop the connection.
            warn!("{} malformed first message, dropping: {}", session.id(), reason);
            session.disconnect();
            Break(())
        }
    }
}

fn process_inbound_msg(session: &mut Session, msg_str: &str) -> ControlFlow<()> {
    match Message::from_str(&msg_str, session.data_dict()) {
        Ok(mut msg) => {
            // next_message has already applied any FIX response (Reject /
            // Logout + disconnect) internally; we only read its signal.
            // Break => session torn down, stop reading; Continue => keep going.
            session.next_message(&mut msg)
        }
        Err(reason) if reason.is_garbled() => {
            // Garbled (bad body length / checksum): drop it — a garbled
            // message can't be trusted enough to reference in a Reject.
            warn!("{} dropping garbled message: {}", session.id(), reason);
            Continue(())
        }
        Err(reason) => {
            // Well-formed but invalid: Reject it (RefSeqNum from the raw
            // message) and keep the connection alive.
            let seq_num = message::seq_num_from_raw(&msg_str)
                .and_then(|sq| sq.parse::<u32>().ok())
                .unwrap_or(0);
            info!("{} rejecting invalid message: {}", session.id(), reason);
            session.reject_message(seq_num, reason);
            Continue(())
        }
    }
}

/// Acceptor-side teardown: log the close and disconnect the session. Takes an
/// already-locked `&mut Session` (the caller owns the lock), so it never locks and
/// cannot deadlock. `Session::disconnect` is idempotent, so calling this after a
/// fatal message already disconnected is safe (a harmless no-op).
fn close_connection(session: &mut Session, addr: SocketAddr) {
    info!("{} event: Connection closed ({})", session.id(), addr);
    session.disconnect();
}

fn wire_display(raw: &str) -> String {
    raw.replace('\x01', "|")
}
