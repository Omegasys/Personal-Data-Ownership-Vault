use std::collections::HashMap;

pub struct AuthMiddleware {
    tokens: HashMap<String, String>, // token -> user_id
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    pub fn generate_token(&mut self, user_id: &str) -> String {
        let token = format!("token_{}", user_id);
        self.tokens.insert(token.clone(), user_id.to_string());
        token
    }

    pub fn verify(&self, token: &str) -> bool {
        self.tokens.contains_key(token)
    }

    pub fn get_user(&self, token: &str) -> Option<String> {
        self.tokens.get(token).cloned()
    }
}
