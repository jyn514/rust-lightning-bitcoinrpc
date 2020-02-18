use serde_json;

pub struct RPCClient;

impl RPCClient {
	pub fn new(user_auth: &str, host_port: &str) -> Self {
            unimplemented!()
        }

	/// params entries must be pre-quoted if appropriate
	/// may_fail is only used to change logging
	pub async fn make_rpc_call(&self, method: &str, params: &[&str], may_fail: bool) -> Result<serde_json::Value, ()> {
            unimplemented!()
	}

	pub async fn get_header(&self, header_hash: &str) -> Result<usize, ()> {
            unimplemented!()
	}
}
