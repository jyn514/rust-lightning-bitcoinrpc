struct RPCClient;

use lightning_net_tokio::Connection;

use tokio::sync::mpsc;

use lightning::chain;
use lightning::ln::{channelmonitor};
use lightning::util::logger::{Logger, Record};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const FEE_PROPORTIONAL_MILLIONTHS: u32 = 10;
const ANNOUNCE_CHANNELS: bool = true;

struct ChannelMonitor {
	monitor: Arc<channelmonitor::SimpleManyChannelMonitor<chain::transaction::OutPoint>>,
	file_prefix: String,
}
impl ChannelMonitor {
	fn load_from_disk(file_prefix: &String) -> Vec<(chain::transaction::OutPoint, channelmonitor::ChannelMonitor)> {
            unimplemented!()
	}

	fn load_from_vec(&self, mut monitors: Vec<(chain::transaction::OutPoint, channelmonitor::ChannelMonitor)>) {
            unimplemented!()
	}
}
impl channelmonitor::ManyChannelMonitor for ChannelMonitor {}

struct LogPrinter {}
impl Logger for LogPrinter {
	fn log(&self, record: &Record) {
            unimplemented!()
	}
}

#[tokio::main]
async fn main() {
	let peer_manager: Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<
                lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>
            >>> = unimplemented!();

	let payment_preimages = Arc::new(Mutex::new(HashMap::new()));
        let event_notify: mpsc::Sender<()> = unimplemented!();

        let line = "hi";
        let stream: tokio::net::TcpStream = unimplemented!();
        if line.len() > 2 && line.as_bytes()[1] == ' ' as u8 && line.as_bytes()[0] == 0x63 {
            let pk: secp256k1::key::PublicKey = unimplemented!();
            if line.as_bytes()[2 + 33*2] == '@' as u8 {
                    tokio::spawn(async move {
                            Connection::setup_outbound(peer_manager, event_notify, pk, stream).await;
                    });
            }
        }
}
