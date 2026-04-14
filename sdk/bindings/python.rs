pub struct PythonBinding;

impl PythonBinding {
    pub fn generate() -> String {
        r#"
import requests

class VaultClient:
    def __init__(self, endpoint):
        self.endpoint = endpoint

    def store(self, data):
        return requests.post(f'{self.endpoint}/store', data=data)

    def get(self):
        return requests.get(f'{self.endpoint}/get')
"#
        .to_string()
    }
}
