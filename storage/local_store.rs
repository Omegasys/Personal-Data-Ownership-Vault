use std::fs;
use std::path::Path;

pub struct LocalStore {
    base_path: String,
}

impl LocalStore {
    pub fn new(base_path: &str) -> Self {
        fs::create_dir_all(base_path).ok();
        Self {
            base_path: base_path.to_string(),
        }
    }

    fn path_for(&self, key: &str) -> String {
        format!("{}/{}", self.base_path, key)
    }

    pub fn put(&self, key: &str, data: &[u8]) {
        let path = self.path_for(key);
        fs::write(path, data).expect("Failed to write file");
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let path = self.path_for(key);
        if Path::new(&path).exists() {
            Some(fs::read(path).expect("Failed to read file"))
        } else {
            None
        }
    }

    pub fn delete(&self, key: &str) {
        let path = self.path_for(key);
        let _ = fs::remove_file(path);
    }
}
