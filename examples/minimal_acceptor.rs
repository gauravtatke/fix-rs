//! Minimal acceptor built on the `fix_rs` engine — the runnable form of the README's
//! library-usage example. Implement `Application`, build a `SessionMap`, start the timer,
//! and run one acceptor thread per bind address.
//!
//! Run with a config that has at least one `connection_type = "acceptor"` session
//! (see `src/FixConfig.example.toml`):  `cargo run --example minimal_acceptor`

use fix_rs::Application;
use fix_rs::core::message::Message;
use fix_rs::errors::{BusinessMsgReject, DonotSend, RejectLogon};
use fix_rs::session::{ConnectionType, SessionId, SessionProperties};
use fix_rs::transport::sync::acceptor::IoAcceptor;
use fix_rs::transport::{SOCKET_ACCEPT_HOST_IP, SessionMap, start_timer};
use std::net::{IpAddr, SocketAddr};

struct MyApp;

impl Application for MyApp {
    fn on_create(&mut self, _sid: &SessionId) {}
    fn on_logon(&mut self, sid: &SessionId) {
        println!("logged on: {sid}");
    }
    fn on_logout(&mut self, _sid: &SessionId) {}

    // Adjust outbound admin messages before they leave (e.g. add Logon credentials).
    fn on_admin_msg_sending(&mut self, _sid: &SessionId, _msg: &mut Message) {}

    // Inbound admin message; return Err(RejectLogon) to reject a Logon.
    fn on_admin_msg_received(
        &mut self,
        _sid: &SessionId,
        _msg: &Message,
    ) -> Result<(), RejectLogon> {
        Ok(())
    }

    // Outbound app message; return Err(DonotSend) to cancel the send.
    fn on_app_msg_sending(
        &mut self,
        _sid: &SessionId,
        _msg: &mut Message,
    ) -> Result<(), DonotSend> {
        Ok(())
    }

    // Inbound app message → return the messages to send back. The engine stamps the header
    // (CompIDs, MsgSeqNum, SendingTime) and serializes each one; you build only the body.
    fn on_app_msg_received(
        &mut self,
        _sid: &SessionId,
        msg: &Message,
    ) -> Result<Vec<Message>, BusinessMsgReject> {
        match msg.get_header_field::<String>(35).as_deref() {
            // NewOrderSingle (35=D) -> ExecutionReport (35=8) "New" ack (body abbreviated).
            Ok("D") => {
                let mut ack = Message::new();
                ack.set_header_field(35, "8");
                ack.set_body_field(39, "0"); // OrdStatus = New
                ack.set_body_field(150, "0"); // ExecType = New
                Ok(vec![ack])
            }
            _ => Ok(vec![]),
        }
    }
}

fn main() {
    let cfg = std::fs::read_to_string("src/FixConfig.toml").expect("read config");
    let props = SessionProperties::from_str(&cfg).expect("parse config");

    // One registry, shared by the timer and every transport.
    let sessions: SessionMap = props
        .sessions()
        .iter()
        .map(|(id, c)| (id.clone(), c.to_session(Box::new(MyApp))))
        .collect();

    start_timer(sessions.clone()); // heartbeats & timeouts on a background thread

    let mut handles = Vec::new();
    for c in props.sessions().values().filter(|c| c.connection_type() == ConnectionType::Acceptor) {
        let addr = SocketAddr::new(
            SOCKET_ACCEPT_HOST_IP.parse::<IpAddr>().unwrap(),
            c.socket_accept_port().unwrap(),
        );
        let map = sessions.clone();
        handles.push(std::thread::spawn(move || {
            IoAcceptor::new(addr, map).start().unwrap(); // blocks accepting connections
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
