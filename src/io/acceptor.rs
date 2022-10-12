use crate::io::*;
use crate::message::SOH;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel as tio_channel, Receiver as TioReceiver, Sender as TioSender};

#[derive(Debug)]
pub struct IoAcceptor {
    bind_addr: SocketAddr,
    socket_to_app_send: TioSender<String>, // for sending message to application

    _app_to_socket_send: TioBroadcastSender<String>, // used by acceptor to recv data from app
                                                     // app_to_socket_send: TioSender<String>,           // used by app code to send data to this
}

impl IoAcceptor {
    pub fn create(
        bind_addr: SocketAddr, to_send: TioSender<String>,
    ) -> (Self, TioBroadcastSender<String>) {
        let self_bind_addr = bind_addr.clone();
        // dropping the receiving end
        let (tx, _) = broadcast::channel::<String>(32);
        let acceptor = IoAcceptor {
            bind_addr: bind_addr,
            socket_to_app_send: to_send,
            _app_to_socket_send: tx.clone(),
            // app_to_socket_send: tx,
        };
        (acceptor, tx)
    }

    pub fn start(&self) {
        let bind_addr = self.bind_addr.clone();
        let socket_to_app_send = self.socket_to_app_send.clone();
        let app_to_socket_send = self._app_to_socket_send.clone();
        tokio::spawn(async move {
            loop {
                let listener = match TcpListener::bind(bind_addr).await {
                    Ok(listener) => {
                        println!("listening on {}", bind_addr);
                        listener
                    }
                    Err(e) => {
                        println!("Error in bind: {:?}", e);
                        continue;
                    }
                };
                let (stream, _) = match listener.accept().await {
                    Ok((stream, remote_addr)) => {
                        println!("accepted connection from {}", remote_addr);
                        (stream, remote_addr)
                    }
                    Err(e) => {
                        println!("Error in accepting connection: {:?}", e);
                        continue;
                    }
                };
                let (owned_read, owned_write) = stream.into_split();
                start_socket_listener_task(owned_read, socket_to_app_send.clone());
                start_app_listner_task(owned_write, app_to_socket_send.subscribe());
            }
        });
    }
}

fn start_socket_listener_task(read_half: OwnedReadHalf, to_app: TioSender<String>) {
    tokio::spawn(async move {
        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        let mut buf_reader = BufReader::new(read_half);
        loop {
            read_message(&mut buf_reader, &mut buf).await;
            let raw_msg = String::from_utf8_lossy(&buf[..buf.len()]).to_string();
            to_app.send(raw_msg).await.unwrap();
            buf.clear();
        }
    });
}

fn start_app_listner_task(
    mut write_half: OwnedWriteHalf, mut from_app: TioBroadcastReceiver<String>,
) {
    tokio::spawn(async move {
        println!("starting internal msg receiv");
        // if there is message to be sent out to remote socket then read and send
        while let Ok(msg) = from_app.recv().await {
            println!("sending {}", &msg);
            let _res = write_half.write_all(msg.as_bytes()).await.unwrap();
            println!("sent {}", &msg);
        }
    });
}

async fn read_message<R: AsyncBufReadExt + Unpin>(reader: &mut R, buf: &mut Vec<u8>) {
    loop {
        let bytes_read = reader.read_until(SOH as u8, buf).await.unwrap();
        // println!("bytes received: {:?}", &buf);
        let slice_start = buf.len() - bytes_read;
        let slice_end = buf.len();
        // last read data
        let byte_slice = &buf[slice_start..slice_end];
        if byte_slice.starts_with(&[49, 48, 61]) {
            // b"10="
            // checksum tag found, break
            break;
        }
    }
}
