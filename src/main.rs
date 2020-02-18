struct RPCClient;

use lightning_net_tokio::Connection;
use lightning::ln::channelmonitor;

struct ChannelMonitor;
impl channelmonitor::ManyChannelMonitor for ChannelMonitor {}

#[tokio::main]
async fn main() {
        tokio::spawn(async move {
                Connection::setup_outbound().await;
        });
}
