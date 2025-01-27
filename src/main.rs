use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use log::{error, info, warn};
use web_server::ThreadPool;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    info!("Server started at port :7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_client(stream);
                })
            }
            Err(e) => {
                error!("Unable to connect: {}", e)
            }
        }
    }

    info!("Shutting down.");
}

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            error!("Failed to read line: {e}");
            return
        },
        None => {
            warn!("Client disconnected or sent no data");
            return
        }
    };

    let current = thread::current();
    match current.name() {
        None => {
            info!("Executing on thread: {:?} for {}", current.id(), request_line)
        }
        Some(name) => {
            info!("Executing on thread: {:?} for {}", name, request_line)
        }
    }

    let (status_line, content) = match &request_line[..] {
        req if req.starts_with("OPTIONS") => (OK_RESPONSE.to_string(), "".to_string()),
        req if req.starts_with("GET / HTTP/1.1") => handle_home_request(req),
        req if req.starts_with("GET /users HTTP/1.1") => handle_users_request(req),
        _ => (NOT_FOUND_RESPONSE.to_string(), "404 not found".to_string())
    };

    stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
}

fn handle_users_request(_request: &str) -> (String, String) {
    thread::sleep(Duration::from_secs(5));
    (OK_RESPONSE.to_string(), "Get Users".to_string())
}

fn handle_home_request(_request: &str) -> (String, String) {
    (OK_RESPONSE.to_string(), "Welcome".to_string())
}

