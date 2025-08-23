use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let input = String::from_utf8_lossy(&buffer[..n]);

                if input.contains("PING") {
                    stream.write_all(b"+PONG\r\n").unwrap();
                }
            }

            Err(_) => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    println!("Connected to Reddis!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}