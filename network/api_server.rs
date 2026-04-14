use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use tiny_http::{Server, Response, Request};

use crate::core::vault::Vault;

pub struct ApiServer {
    vault: Arc<Mutex<Vault>>,
    addr: String,
}

impl ApiServer {
    pub fn new(vault: Arc<Mutex<Vault>>, addr: &str) -> Self {
        Self {
            vault,
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) {
        let server = Server::http(&self.addr).expect("Failed to start server");

        println!("Vault API running at {}", self.addr);

        for request in server.incoming_requests() {
            self.handle_request(request);
        }
    }

    fn handle_request(&self, req: Request) {
        let url = req.url().to_string();

        if url.starts_with("/store") {
            let mut vault = self.vault.lock().unwrap();
            vault.store_data("api_key", b"example_data");
            let _ = req.respond(Response::from_string("stored"));
        } else if url.starts_with("/get") {
            let vault = self.vault.lock().unwrap();
            let data = vault.retrieve_data("api_key");
            let resp = format!("data: {:?}", data);
            let _ = req.respond(Response::from_string(resp));
        } else {
            let _ = req.respond(Response::from_string("not found").with_status_code(404));
        }
    }
}
