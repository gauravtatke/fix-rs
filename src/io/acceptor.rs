use crate::io::fix_message_reader::FixMessageReader;
use crate::io::tcp_responder::TcpResponder;
use crate::message::{self, Message};
use crate::network::SessionMap;
use log::{info, warn};
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
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
    if let Some(s_arc) = session_map.get(&reverse_id) {
        info!("{} incoming: {}", reverse_id, wire_display(&first_msg_str));
        {
            let mut session = s_arc.lock().unwrap();
            session.set_responder(Box::new(TcpResponder::new(stream)));
            let mut first_msg_result = Message::from_str(&first_msg_str, session.dictionary());
            match first_msg_result {
                Ok(mut first_msg) => session.next_message(&mut first_msg)?,
                Err(reason) => {
                    session.disconnect();
                    return Err(From::from(reason));
                }
            }
        }
        while let Ok(msg_str) = fix_msg_reader.read_message() {
            info!("{} incoming: {}", reverse_id, wire_display(&msg_str));
            let mut session = s_arc.lock().unwrap();
            match Message::from_str(&msg_str, session.dictionary()) {
                Ok(mut msg) => {
                    session.next_message(&mut msg)?;
                }
                Err(reason) if reason.is_garbled() => {
                    // Garbled (bad body length / checksum): drop it — a garbled
                    // message can't be trusted enough to reference in a Reject.
                    warn!("{} dropping garbled message: {}", reverse_id, reason);
                    continue;
                }
                Err(reason) => {
                    // Well-formed but invalid: Reject it (RefSeqNum from the raw
                    // message) and keep the connection alive.
                    let seq_num = message::seq_num_from_raw(&msg_str)
                        .and_then(|sq| sq.parse::<u32>().ok())
                        .unwrap_or(0);
                    info!("{} rejecting invalid message: {}", reverse_id, reason);
                    session.reject_message(seq_num, reason);
                    continue;
                }
            }
        }
        info!("{} event: Connection closed ({})", reverse_id, peer_addr);
        let mut session = s_arc.lock().unwrap();
        session.disconnect();
    } else {
        info!("No session found for {}, dropping connection from {}", reverse_id, peer_addr);
    }
    Ok(())
}

fn wire_display(raw: &str) -> String {
    raw.replace('\x01', "|")
}
