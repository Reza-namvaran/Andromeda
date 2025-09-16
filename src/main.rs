mod resp;
mod commands;

use resp::parse_resp;
use commands::handle_command;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

type DB = Arc<Mutex<HashMap<String, String>>>;

fn handle_client(stream: TcpStream, db: DB) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        match parse_resp(&mut reader) {
            Ok(resp) => {
                if let Some(reply) = handle_command(resp, &db) {
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
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Server listening on port 6379...");

    let db: DB = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                thread::spawn(|| handle_client(stream, db));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
