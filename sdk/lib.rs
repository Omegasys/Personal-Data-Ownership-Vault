pub mod bindings;

pub struct VaultSDK {
    pub endpoint: String,
}

impl VaultSDK {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn info(&self) -> String {
        format!("VaultSDK connected to {}", self.endpoint)
    }
}
