use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use log::{error, info};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    info!("Server started at port :7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                error!("Unable to connect: {}", e)
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                req if req.starts_with("OPTIONS") => (OK_RESPONSE.to_string(), "".to_string()),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 not found".to_string())
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => error!("Unable to read stream: {}", e)
    }
}

