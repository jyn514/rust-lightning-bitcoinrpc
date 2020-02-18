struct RPCClient;

use lightning_net_tokio::Connection;

use tokio::sync::mpsc;
use lightning::ln::{channelmonitor, channelmanager, peer_handler};
use lightning::chain::keysinterface;

use std::sync::Arc;

struct ChannelMonitor;
impl channelmonitor::ManyChannelMonitor for ChannelMonitor {}

#[tokio::main]
async fn main() {
        tokio::spawn(async move {
                Connection::setup_outbound().await;
        });
}
