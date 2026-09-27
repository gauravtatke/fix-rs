use crate::transport::sync::reader::FixMessageReader;
use crate::core::message;
use crate::core::message::Message;
use crate::session::Session;
use log::{info, warn};
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::ops::ControlFlow;
use std::ops::ControlFlow::Continue;
use std::sync::{Arc, Mutex};

/// Pumps messages from an already-established connection until the peer goes away,
/// then tears the session down. Shared by both roles: the acceptor calls it after it
/// has identified the session and dispatched the first (Logon) message; the initiator
/// calls it after it has dialed out and wired the responder.
///
/// Takes the `Arc<Mutex<Session>>` (not an already-locked `&mut Session`) on purpose —
/// it must lock only for the dispatch of each message, never across the blocking
/// `read_message()`, or the timer thread could never tick (no heartbeats) while a read
/// is parked. `session_id` is the session's own-perspective id, used only for logging.
pub(crate) fn run_connection(
    s_arc: Arc<Mutex<Session>>,
    session_id: &str,
    mut reader: FixMessageReader<TcpStream>,
    peer_addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    while let Ok(msg_str) = reader.read_message() {
        info!("{} incoming: {}", session_id, wire_display(&msg_str));
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

// pub(crate) so the in-process raw-input harness (session_tests, 7.6a) can drive
// this exact inbound seam — parse -> garbled-drop / Reject / dispatch — without a
// socket. Crate-internal only; not part of any external API.
pub(crate) fn process_inbound_msg(session: &mut Session, msg_str: &str) -> ControlFlow<()> {
    match Message::from_str(msg_str, session.data_dict()) {
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
            let seq_num = message::seq_num_from_raw(msg_str)
                .and_then(|sq| sq.parse::<u32>().ok())
                .unwrap_or(0);
            info!("{} rejecting invalid message: {}", session.id(), reason);
            session.reject_message(seq_num, reason);
            Continue(())
        }
    }
}

/// Connection teardown (both roles): log the close and disconnect the session. Takes
/// an already-locked `&mut Session` (the caller owns the lock), so it never locks and
/// cannot deadlock. `Session::disconnect` is idempotent, so calling this after a
/// fatal message already disconnected is safe (a harmless no-op).
pub(crate) fn close_connection(session: &mut Session, addr: SocketAddr) {
    info!("{} event: Connection closed ({})", session.id(), addr);
    session.disconnect();
}

pub(crate) fn wire_display(raw: &str) -> String {
    raw.replace('\x01', "|")
}
