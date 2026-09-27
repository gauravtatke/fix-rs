use crate::session::Responder;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::Mutex;

/// Production `Responder` that writes FIX messages to a TCP socket.
/// Uses `Mutex<TcpStream>` because `Responder::send` takes `&self` — multiple
/// threads (session logic, timer) may call it concurrently.
pub struct TcpResponder {
    stream: Mutex<TcpStream>,
}

impl TcpResponder {
    pub fn new(stream: TcpStream) -> TcpResponder {
        Self {
            stream: Mutex::new(stream),
        }
    }
}

impl Responder for TcpResponder {
    fn send(&self, msg: &str) -> bool {
        self.stream.lock().unwrap().write_all(msg.as_bytes()).is_ok()
    }

    fn disconnect(&self) {
        let _ = self.stream.lock().unwrap().shutdown(Shutdown::Both);
    }
}

#[cfg(test)]
mod tcp_responder_tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;

    fn make_connected_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let client = TcpStream::connect(addr).unwrap();
        let server = listener.accept().unwrap().0;
        (client, server)
    }

    #[test]
    fn test_send_writes_bytes_to_stream() {
        let (writer, mut reader) = make_connected_pair();
        let responder = TcpResponder::new(writer);

        let result = responder.send("8=FIX.4.3\x0110=128\x01");
        assert!(result);

        let mut buf = vec![0u8; 256];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"8=FIX.4.3\x0110=128\x01");
    }

    #[test]
    fn test_send_multiple_messages() {
        let (writer, mut reader) = make_connected_pair();
        let responder = TcpResponder::new(writer);

        responder.send("msg1\x01");
        responder.send("msg2\x01");
        responder.disconnect();

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, b"msg1\x01msg2\x01");
    }

    #[test]
    fn test_disconnect_causes_eof_on_reader() {
        let (writer, mut reader) = make_connected_pair();
        let responder = TcpResponder::new(writer);

        responder.disconnect();

        let mut buf = vec![0u8; 256];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_send_after_disconnect_returns_false() {
        let (writer, _reader) = make_connected_pair();
        let responder = TcpResponder::new(writer);

        responder.disconnect();
        let result = responder.send("should fail");
        assert!(!result);
    }

    #[test]
    fn test_disconnect_twice_does_not_panic() {
        let (writer, _reader) = make_connected_pair();
        let responder = TcpResponder::new(writer);

        responder.disconnect();
        responder.disconnect();
    }
}
