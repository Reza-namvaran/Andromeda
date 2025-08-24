mod resp;
mod commands;

use resp::parse_resp;
use commands::handle_command;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        match parse_resp(&mut reader) {
            Ok(resp) => {
                if let Some(reply) = handle_command(resp) {
                    if writer.write_all(reply.as_bytes()).is_err() {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server listening on port 6379...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
