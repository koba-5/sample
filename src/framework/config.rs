mod server;

use crate::framework::config::server::Server;

pub struct Config {
    pub server: Server,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server: Server { port: 8000 },
        }
    }
}
