use std::collections::HashMap;

use crate::rpc_client::RpcClient;

pub struct RpcClientManager {
    rpc_clients: HashMap<u64, RpcClient>,
}

impl RpcClientManager {
    pub fn new() -> Self {
        Self {
            rpc_clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, rpc_client: RpcClient) {
        println!(
            "[RPC Client Manager] add client instance {} ({} total instances registered)",
            &rpc_client.client_id,
            self.rpc_clients.len() + 1
        );
        self.rpc_clients.insert(rpc_client.client_id, rpc_client);
    }

    pub fn get_client_mut(&mut self, client_id: u64) -> Option<&mut RpcClient> {
        self.rpc_clients.get_mut(&client_id)
    }

    pub fn clear_all_activities(&mut self) {
        let keys: Vec<_> = self.rpc_clients.keys().cloned().collect();

        for key in keys {
            match self.rpc_clients.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().clear_activity();
                }
                _ => {}
            }
        }
    }
}
