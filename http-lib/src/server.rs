use std::net::{SocketAddr, TcpListener};
use crate::http::HttpMethod;
use std::io::Result;

pub struct Route {
    http_method: HttpMethod,
    path: String,
}

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn start(&self) -> Result<()> {
        let address = self.addr;
        let listener = TcpListener::bind(address)?;

        Ok(())
    }
}