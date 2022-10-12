pub(crate) use tokio::sync::{broadcast, mpsc};

pub(crate) mod acceptor;

pub type TioBroadcastSender<T> = broadcast::Sender<T>;
pub type TioBroadcastReceiver<T> = broadcast::Receiver<T>;
