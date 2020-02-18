mod rpc_client;
use rpc_client::*;

mod utils;
use utils::*;

mod chain_monitor;
use chain_monitor::*;

use lightning_net_tokio::Connection;

use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;

use rand::Rng;

use lightning::chain;
use lightning::chain::chaininterface;
use lightning::chain::keysinterface::{KeysInterface, KeysManager};
use lightning::ln::{peer_handler, router, channelmanager, channelmonitor};
use lightning::ln::channelmanager::{PaymentHash, PaymentPreimage};
use lightning::util::logger::{Logger, Record};
use lightning::util::ser::ReadableArgs;
use lightning::util::config;

use bitcoin::util::bip32;
use bitcoin::blockdata;
use bitcoin::network::constants;

use bitcoin_hashes::sha256d::Hash as Sha256dHash;

use std::env;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::time::{Duration, SystemTime};
use std::fs;

const FEE_PROPORTIONAL_MILLIONTHS: u32 = 10;
const ANNOUNCE_CHANNELS: bool = true;

struct EventHandler {
	secp_ctx: Secp256k1<secp256k1::All>,
	network: constants::Network,
	file_prefix: String,
	rpc_client: Arc<RPCClient>,
	peer_manager: peer_handler::SimpleArcPeerManager<lightning_net_tokio::SocketDescriptor, ChannelMonitor>,
	channel_manager: channelmanager::SimpleArcChannelManager<ChannelMonitor>,
	monitor: Arc<channelmonitor::SimpleManyChannelMonitor<chain::transaction::OutPoint>>,
	broadcaster: Arc<dyn chain::chaininterface::BroadcasterInterface>,
	txn_to_broadcast: Mutex<HashMap<chain::transaction::OutPoint, blockdata::transaction::Transaction>>,
	payment_preimages: Arc<Mutex<HashMap<PaymentHash, PaymentPreimage>>>,
}
impl EventHandler {
	async fn setup(network: constants::Network, file_prefix: String, rpc_client: Arc<RPCClient>, peer_manager: peer_handler::SimpleArcPeerManager<lightning_net_tokio::SocketDescriptor, ChannelMonitor>, monitor: Arc<channelmonitor::SimpleManyChannelMonitor<chain::transaction::OutPoint>>, channel_manager: channelmanager::SimpleArcChannelManager<ChannelMonitor>, broadcaster: Arc<dyn chain::chaininterface::BroadcasterInterface>, payment_preimages: Arc<Mutex<HashMap<PaymentHash, PaymentPreimage>>>) -> mpsc::Sender<()> {
            unimplemented!()
	}

	async fn check_handle_event(us: &Arc<Self>, self_sender: &mut mpsc::Sender<()>) {
            unimplemented!()
	}
}

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
	let rpc_client = {
		let path = env::args().skip(1).next().unwrap();
		let path_parts: Vec<&str> = path.split('@').collect();
		if path_parts.len() != 2 {
			println!("Bad RPC URL provided");
			return;
		}
		Arc::new(RPCClient::new(path_parts[0], path_parts[1]))
	};

	let mut network = constants::Network::Bitcoin;
	let secp_ctx = Secp256k1::new();
	let fee_estimator = Arc::new(FeeEstimator::new());
	let data_path = env::args().skip(2).next().unwrap();
	let port = 9735_u16;
	let logger = Arc::new(LogPrinter {});

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

	let channel_manager = if let Ok(mut f) = fs::File::open(data_path.clone() + "/manager_data") {
		let (last_block_hash, manager) = {
			let mut monitors_refs = HashMap::new();
			for (outpoint, monitor) in monitors_loaded.iter_mut() {
				monitors_refs.insert(*outpoint, monitor);
			}
			<(Sha256dHash, channelmanager::SimpleArcChannelManager<ChannelMonitor>)>::read(&mut f, channelmanager::ChannelManagerReadArgs {
				keys_manager: keys.clone(),
				fee_estimator: fee_estimator.clone(),
				monitor: monitor.clone(),
				//chain_monitor: chain_monitor.clone(),
				tx_broadcaster: chain_monitor.clone(),
				logger: logger.clone(),
				default_config: config,
				channel_monitors: &mut monitors_refs,
			}).expect("Failed to deserialize channel manager")
		};
		monitor.load_from_vec(monitors_loaded);
		//TODO: Rescan
		manager
	} else {
		if !monitors_loaded.is_empty() {
			panic!("Found some channel monitors but no channel state!");
		}
		Arc::new(channelmanager::ChannelManager::new(network, fee_estimator.clone(), monitor.clone(), chain_monitor.clone(), logger.clone(), keys.clone(), config, 0).unwrap()) //TODO: Get blockchain height
	};
	block_notifier.register_listener(Arc::clone(&(channel_manager.clone() as Arc<dyn chaininterface::ChainListener>)));
	let router = Arc::new(router::Router::new(PublicKey::from_secret_key(&secp_ctx, &keys.get_node_secret()), chain_monitor.clone(), logger.clone()));

	let mut ephemeral_data = [0; 32];
	rand::thread_rng().fill_bytes(&mut ephemeral_data);
	let peer_manager = Arc::new(peer_handler::PeerManager::new(peer_handler::MessageHandler {
		chan_handler: channel_manager.clone(),
		route_handler: router.clone(),
	}, keys.get_node_secret(), &ephemeral_data, logger.clone()));

	let payment_preimages = Arc::new(Mutex::new(HashMap::new()));
	let mut event_notify = EventHandler::setup(network, data_path, rpc_client.clone(), peer_manager.clone(), monitor.monitor.clone(), channel_manager.clone(), chain_monitor.clone(), payment_preimages.clone()).await;

        let line = "hi";
        if line.len() > 2 && line.as_bytes()[1] == ' ' as u8 && line.as_bytes()[0] == 0x63 {
            if let Some(pk) = hex_to_compressed_pubkey(line.split_at(2).1) {
                if line.as_bytes()[2 + 33*2] == '@' as u8 {
                        let parse_res: Result<std::net::SocketAddr, _> = line.split_at(2 + 33*2 + 1).1.parse();
                        if let Ok(addr) = parse_res {
                                if let Ok(stream) = std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(10)) {
                                    tokio::spawn(async move {
                                            Connection::setup_outbound(peer_manager, event_notify, pk,
                                                    tokio::net::TcpStream::from_std(stream).unwrap()).await;
                                    });
                                }
                        }
                }
            }
        }
}
