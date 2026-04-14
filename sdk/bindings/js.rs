pub struct JSBinding;

impl JSBinding {
    pub fn generate() -> String {
        r#"
class VaultClient {
    constructor(endpoint) {
        this.endpoint = endpoint;
    }

    async store(data) {
        return fetch(`${this.endpoint}/store`, {
            method: "POST",
            body: data
        });
    }

    async get() {
        return fetch(`${this.endpoint}/get`);
    }
}

export default VaultClient;
"#
        .to_string()
    }
}
