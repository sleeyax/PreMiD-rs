use std::collections::HashMap;

use tracing::info;

use crate::rpc_client::RpcClient;

pub struct RpcClientManager {
    rpc_clients: HashMap<String, RpcClient>,
}

impl RpcClientManager {
    pub fn new() -> Self {
        Self {
            rpc_clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, rpc_client: RpcClient) {
        info!(
            "add client instance {} ({} total instances registered)",
            rpc_client.get_client_id(),
            self.rpc_clients.len() + 1
        );
        self.rpc_clients
            .insert(rpc_client.get_client_id(), rpc_client);
    }

    pub fn get_client_mut(&mut self, client_id: String) -> Option<&mut RpcClient> {
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
