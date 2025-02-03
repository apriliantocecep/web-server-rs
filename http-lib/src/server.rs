use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use crate::http::HttpMethod;
use std::io::Result;
use log::info;

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
        info!("Server listening on {}", address.to_string());

        Ok(())
    }
}