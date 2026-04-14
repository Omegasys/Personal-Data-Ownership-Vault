use std::collections::HashMap;

pub struct PermissionManager {
    rules: HashMap<String, Vec<String>>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn grant(&mut self, resource: &str, entity: &str) {
        self.rules
            .entry(resource.to_string())
            .or_default()
            .push(entity.to_string());
    }

    pub fn check(&self, resource: &str, entity: &str) -> bool {
        self.rules
            .get(resource)
            .map_or(false, |entities| entities.contains(&entity.to_string()))
    }
}
