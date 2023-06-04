use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");
        if bytes_read == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message: {}", message);

        let response = format!("Server received: {}", message);
        stream.write_all(response.as_bytes()).expect("Failed to write to client");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
