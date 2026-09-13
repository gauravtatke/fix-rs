use crate::message::SOH;
use std::io::{self, BufRead, BufReader, Read};

const CHECKSUM_PREFIX: &[u8; 3] = b"10=";

/// Reads complete FIX messages from a byte stream by accumulating SOH-delimited
/// fields until the checksum tag (`10=`) is found. Generic over `Read` so it works
/// with both `TcpStream` (production) and `Cursor<Vec<u8>>` (tests).
pub struct FixMessageReader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> FixMessageReader<R> {
    pub fn new(reader: R) -> FixMessageReader<R> {
        FixMessageReader {
            reader: BufReader::new(reader),
        }
    }

    /// Blocks until a complete FIX message is available, then returns it as a String.
    /// Returns `Err(UnexpectedEof)` if the stream closes before a checksum tag arrives.
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut buffer: Vec<u8> = Vec::with_capacity(16);
        loop {
            let count = self.reader.read_until(SOH as u8, &mut buffer)?;
            if count == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "connection closed mid-message",
                ));
            }
            if buffer[buffer.len() - count..].starts_with(CHECKSUM_PREFIX) {
                return Ok(String::from_utf8_lossy(&buffer).to_string());
            }
        }
    }
}

#[cfg(test)]
mod fix_message_reader_tests {
    use super::*;
    use std::io::Cursor;

    fn make_fix_msg(body: &str, checksum: &str) -> String {
        format!(
            "8=FIX.4.3{SOH}9={len}{SOH}{body}{SOH}10={checksum}{SOH}",
            SOH = SOH,
            len = body.len(),
            body = body,
            checksum = checksum,
        )
    }

    #[test]
    fn test_single_complete_message() {
        let msg = make_fix_msg("35=A", "128");
        let cursor = Cursor::new(msg.as_bytes().to_vec());
        let mut reader = FixMessageReader::new(cursor);

        let result = reader.read_message().unwrap();
        assert_eq!(result, msg);
    }

    #[test]
    fn test_two_messages_back_to_back() {
        let msg1 = make_fix_msg("35=A", "128");
        let msg2 = make_fix_msg("35=0", "200");
        let combined = format!("{}{}", msg1, msg2);
        let cursor = Cursor::new(combined.as_bytes().to_vec());
        let mut reader = FixMessageReader::new(cursor);

        let result1 = reader.read_message().unwrap();
        assert_eq!(result1, msg1);

        let result2 = reader.read_message().unwrap();
        assert_eq!(result2, msg2);
    }

    #[test]
    fn test_eof_mid_message_returns_error() {
        let truncated = format!("8=FIX.4.3{SOH}9=5{SOH}35=A{SOH}", SOH = SOH);
        let cursor = Cursor::new(truncated.as_bytes().to_vec());
        let mut reader = FixMessageReader::new(cursor);

        let err = reader.read_message().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_eof_before_any_bytes_returns_error() {
        let cursor = Cursor::new(Vec::<u8>::new());
        let mut reader = FixMessageReader::new(cursor);

        let err = reader.read_message().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
    }
}
