use reqwest;

pub struct VaultClient {
    base_url: String,
}

impl VaultClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn store(&self, data: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/store", self.base_url);

        let resp = reqwest::Client::new()
            .post(url)
            .body(data.to_string())
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn get(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/get", self.base_url);

        let resp = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}
