use std::io::{Read};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server started at port :7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                println!("Unable to connect: {}", e)
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

            println!("{}", request);
        }
        Err(e) => eprintln!("Unable to read stream: {}", e)
    }
}

