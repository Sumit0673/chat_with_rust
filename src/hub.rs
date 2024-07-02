use tokio::sync::broadcast;
use tokio::time::{self, Duration};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::UnboundedReceiver;
use futures::StreamExt;

use crate::proto::InputParcel;

pub struct Hub {
    sender: broadcast::Sender<InputParcel>,
}

impl Hub {
    pub fn new(_options: HubOptions) -> Self {
        let (sender, _) = broadcast::channel(100);
        Hub { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<InputParcel> {
        self.sender.subscribe()
    }

    pub async fn on_disconnect(&self, _client_id: u64) {
        // Your on_disconnect implementation
    }

    pub async fn run(&self, input_receiver: UnboundedReceiver<InputParcel>) {
        let mut stream = UnboundedReceiverStream::new(input_receiver);

        let alive_interval = Duration::from_secs(5);
        let ticking_alive = async move {
            loop {
                time::sleep(alive_interval).await;
                // Do something on interval
            }
        };

        tokio::pin!(ticking_alive);

        tokio::select! {
            _ = ticking_alive => {},
            _ = stream.next() => {},
        }
    }
}

pub struct HubOptions {
    pub alive_interval: Option<Duration>,
}
