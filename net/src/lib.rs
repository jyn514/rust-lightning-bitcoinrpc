use tokio::io::{WriteHalf, AsyncWriteExt, Sink};

use std::sync::Mutex;

pub struct Connection {
    writer: WriteHalf<Sink>,
}

impl Connection {
    pub async fn setup_outbound() {
        let us: Mutex<Self> = unimplemented!();
        us.lock().unwrap().writer.write_all(b"").await;
    }
}
