mod resp;
mod commands;

use commands::{handle_command, Db};
use resp::parse_resp;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// Handle a client connection
fn handle_client(stream: TcpStream, db: Db, log: Arc<Mutex<File>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        match parse_resp(&mut reader) {
            Ok(resp) => {
                if let Some(reply) = handle_command(resp, &db, &log) {
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

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let log_path = "db.log";
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(log_path)?;

    // Replay the log file to restore DB state
    {
        let mut reader = BufReader::new(&log_file);
        commands::replay_log(&mut reader, &db)?;
    }

    let log = Arc::new(Mutex::new(
        OpenOptions::new().append(true).open(log_path)?,
    ));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                let log = Arc::clone(&log);
                thread::spawn(|| handle_client(stream, db, log));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
