struct RPCClient;

use lightning_net_tokio::Connection;

pub trait T: Send + Sync {}

struct ChannelMonitor;
impl T for ChannelMonitor {}

#[tokio::main]
async fn main() {
        tokio::spawn(async move {
                Connection::setup_outbound().await;
        });
}
