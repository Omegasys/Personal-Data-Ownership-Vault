use std::collections::HashMap;

#[derive(Clone)]
pub struct Metadata {
    pub chunks: Vec<String>,
    pub created_at: u64,
}

pub struct Index {
    map: HashMap<String, Metadata>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, metadata: Metadata) {
        self.map.insert(id, metadata);
    }

    pub fn get(&self, id: &str) -> Option<&Metadata> {
        self.map.get(id)
    }

    pub fn delete(&mut self, id: &str) {
        self.map.remove(id);
    }

    pub fn list(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
}
