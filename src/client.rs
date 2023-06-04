use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

fn handle_server(mut stream: TcpStream) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        stream.write_all(input.as_bytes()).expect("Failed to write to server");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Server response: {}", response);
    }
}

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => {
            let handle = thread::spawn(move || {
                handle_server(stream);
            });
            handle.join().expect("Server thread panicked");
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
}
