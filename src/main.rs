struct RPCClient;

mod chain_monitor;
use chain_monitor::*;

use lightning_net_tokio::Connection;

use tokio::sync::mpsc;

use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;

use lightning::chain;
use lightning::chain::chaininterface;
use lightning::chain::keysinterface::{KeysInterface, KeysManager};
use lightning::ln::{peer_handler, router, channelmanager, channelmonitor};
use lightning::ln::channelmanager::{PaymentHash, PaymentPreimage};
use lightning::util::logger::{Logger, Record};
use lightning::util::config;

use bitcoin::util::bip32;
use bitcoin::network::constants;

use std::env;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::time::{Duration, SystemTime};

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
	let rpc_client: Arc<RPCClient> = unimplemented!();
	let mut network = constants::Network::Bitcoin;
	let secp_ctx = Secp256k1::new();
	let fee_estimator = Arc::new(FeeEstimator::new());
	let data_path = env::args().skip(2).next().unwrap();
	let port = 9735_u16;
	let logger: Arc<LogPrinter> = unimplemented!();

	let our_node_seed = [0; 32];
	let cur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
	let keys = Arc::new(KeysManager::new(&our_node_seed, network, logger.clone(), cur.as_secs(), cur.subsec_nanos()));
	let (import_key_1, import_key_2) = bip32::ExtendedPrivKey::new_master(network, &our_node_seed).map(|extpriv| {
		(extpriv.ckd_priv(&secp_ctx, bip32::ChildNumber::from_hardened_idx(1).unwrap()).unwrap().private_key.key,
		 extpriv.ckd_priv(&secp_ctx, bip32::ChildNumber::from_hardened_idx(2).unwrap()).unwrap().private_key.key)
	}).unwrap();
	let chain_monitor = Arc::new(ChainInterface::new(rpc_client.clone(), network, logger.clone()));
	let block_notifier: chaininterface::BlockNotifierArc = Arc::new(chaininterface::BlockNotifier::new(chain_monitor.clone()));

	let mut monitors_loaded = ChannelMonitor::load_from_disk(&(data_path.clone() + "/monitors"));
	let monitor = Arc::new(ChannelMonitor {
		monitor: Arc::new(channelmonitor::SimpleManyChannelMonitor::new(chain_monitor.clone(), chain_monitor.clone(), logger.clone(), fee_estimator.clone())),
		file_prefix: data_path.clone() + "/monitors",
	});

	let mut config: config::UserConfig = Default::default();
	config.channel_options.fee_proportional_millionths = FEE_PROPORTIONAL_MILLIONTHS;
	config.channel_options.announced_channel = ANNOUNCE_CHANNELS;

	let channel_manager = {
		Arc::new(channelmanager::ChannelManager::new(network, fee_estimator.clone(), monitor.clone(), chain_monitor.clone(), logger.clone(), keys.clone(), config, 0).unwrap()) //TODO: Get blockchain height
	};
	block_notifier.register_listener(Arc::clone(&(channel_manager.clone() as Arc<dyn chaininterface::ChainListener>)));
	let router = Arc::new(router::Router::new(PublicKey::from_secret_key(&secp_ctx, &keys.get_node_secret()), chain_monitor.clone(), logger.clone()));

	let mut ephemeral_data = [0; 32];
	let peer_manager = Arc::new(peer_handler::PeerManager::new(peer_handler::MessageHandler {
		chan_handler: channel_manager.clone(),
		route_handler: router.clone(),
	}, keys.get_node_secret(), &ephemeral_data, logger.clone()));

	let payment_preimages = Arc::new(Mutex::new(HashMap::new()));
        let event_notify: mpsc::Sender<()> = unimplemented!();

        let line = "hi";
        if line.len() > 2 && line.as_bytes()[1] == ' ' as u8 && line.as_bytes()[0] == 0x63 {
            let pk: secp256k1::key::PublicKey = unimplemented!();
            if line.as_bytes()[2 + 33*2] == '@' as u8 {
                    let stream: std::net::TcpStream = unimplemented!();
                    tokio::spawn(async move {
                            Connection::setup_outbound(peer_manager, event_notify, pk,
                                    tokio::net::TcpStream::from_std(stream).unwrap()).await;
                    });
            }
        }
}
