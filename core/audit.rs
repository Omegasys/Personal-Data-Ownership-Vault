use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuditLog {
    entries: Vec<String>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn log(&mut self, event: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = format!("[{}] {}", timestamp, event);
        self.entries.push(entry);
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.entries
    }
}
