use std::fs;

#[derive(Debug)]
pub struct Config {
    pub data_dir: String,
    pub api_port: u16,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .expect("Failed to read config file");

        // VERY simple parser (replace with serde later)
        let mut data_dir = String::from("./data");
        let mut api_port = 8080;

        for line in content.lines() {
            if line.starts_with("data_dir=") {
                data_dir = line.replace("data_dir=", "");
            }
            if line.starts_with("api_port=") {
                api_port = line.replace("api_port=", "").parse().unwrap_or(8080);
            }
        }

        Self { data_dir, api_port }
    }
}
