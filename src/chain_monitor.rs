use crate::RPCClient;

use bitcoin::Network;
use bitcoin_hashes::sha256d::Hash as Sha256dHash;

use lightning::chain::chaininterface;
use lightning::util::logger::Logger;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FeeEstimator;
impl FeeEstimator {
	pub fn new() -> Self {
            unimplemented!()
	}
}
impl chaininterface::FeeEstimator for FeeEstimator {
	fn get_est_sat_per_1000_weight(&self, conf_target: chaininterface::ConfirmationTarget) -> u64 {
            unimplemented!()
	}
}

pub struct ChainInterface {
	util: chaininterface::ChainWatchInterfaceUtil,
	txn_to_broadcast: Mutex<HashMap<Sha256dHash, bitcoin::blockdata::transaction::Transaction>>,
	rpc_client: Arc<RPCClient>,
}
impl ChainInterface {
	pub fn new(rpc_client: Arc<RPCClient>, network: Network, logger: Arc<dyn Logger>) -> Self {
            unimplemented!()
	}
}
impl chaininterface::ChainWatchInterface for ChainInterface {}
impl chaininterface::BroadcasterInterface for ChainInterface {}
