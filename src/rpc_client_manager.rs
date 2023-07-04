use crate::rpc_client::RpcClient;

pub struct RpcClientManager {
    rpc_clients: Vec<RpcClient>,
}

impl RpcClientManager {
    pub fn new() -> Self {
        Self {
            rpc_clients: Vec::new(),
        }
    }

    pub fn add_client_instance(&mut self, rpc_client: RpcClient) {
        println!(
            "[RPC Client Manager] add client instance {} ({} total instances registered)",
            &rpc_client.client_id,
            self.rpc_clients.len() + 1
        );
        self.rpc_clients.push(rpc_client);
    }

    pub fn add_client(&mut self, client_id: u64) {
        self.add_client_instance(RpcClient::new(client_id));
    }

    pub fn remove_client(&mut self, client_id: u64) {
        let index = self
            .rpc_clients
            .iter()
            .position(|client| client.client_id == client_id)
            .unwrap();
        self.rpc_clients.remove(index);
    }

    pub fn get_client(&self, client_id: u64) -> Option<&RpcClient> {
        self.rpc_clients
            .iter()
            .find(|client| client.client_id == client_id)
    }

    pub fn get_client_mut(&mut self, client_id: u64) -> Option<&mut RpcClient> {
        self.rpc_clients
            .iter_mut()
            .find(|client| client.client_id == client_id)
    }

    pub fn get_clients(&self) -> &Vec<RpcClient> {
        &self.rpc_clients
    }

    pub fn get_clients_mut(&mut self) -> &mut Vec<RpcClient> {
        &mut self.rpc_clients
    }
}
