pub struct P2PNode {
    pub node_id: String,
}

impl P2PNode {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
        }
    }

    pub fn broadcast(&self, message: &str) {
        println!("[P2P:{}] broadcast: {}", self.node_id, message);
    }

    pub fn receive(&self, message: &str) {
        println!("[P2P:{}] received: {}", self.node_id, message);
    }
}
