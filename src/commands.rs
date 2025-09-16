use crate::resp::RespType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<HashMap<String, String>>>;

/// Handles a RESP command and interacts with the DB
pub fn handle_command(resp: RespType, db: &Db) -> Option<String> {
    match resp {
        RespType::Array(Some(items)) => {
            if items.is_empty() {
                return Some("-ERR empty command\r\n".to_string());
            }

            match &items[0] {
                RespType::BulkString(Some(cmd)) => {
                    match cmd.to_uppercase().as_str() {
                        "PING" => Some("+PONG\r\n".to_string()),

                        "ECHO" => {
                            if items.len() > 1 {
                                if let RespType::BulkString(Some(msg)) = &items[1] {
                                    Some(format!("${}\r\n{}\r\n", msg.len(), msg))
                                } else {
                                    Some("-ERR invalid argument\r\n".to_string())
                                }
                            } else {
                                Some("-ERR wrong number of arguments for 'echo'\r\n".to_string())
                            }
                        }

                        "SET" => {
                            if items.len() < 3 {
                                return Some("-ERR wrong number of arguments for 'set'\r\n".to_string());
                            }
                            if let (RespType::BulkString(Some(key)), RespType::BulkString(Some(val))) =
                                (&items[1], &items[2])
                            {
                                let mut db = db.lock().unwrap();
                                db.insert(key.clone(), val.clone());
                                Some("+OK\r\n".to_string())
                            } else {
                                Some("-ERR invalid arguments\r\n".to_string())
                            }
                        }

                        "GET" => {
                            if items.len() < 2 {
                                return Some("-ERR wrong number of arguments for 'get'\r\n".to_string());
                            }
                            if let RespType::BulkString(Some(key)) = &items[1] {
                                let db = db.lock().unwrap();
                                match db.get(key) {
                                    Some(val) => Some(format!("${}\r\n{}\r\n", val.len(), val)),
                                    None => Some("$-1\r\n".to_string()), // RESP nil
                                }
                            } else {
                                Some("-ERR invalid key\r\n".to_string())
                            }
                        }

                        "DEL" => {
                            if items.len() < 2 {
                                return Some("-ERR wrong number of arguments for 'del'\r\n".to_string());
                            }
                            if let RespType::BulkString(Some(key)) = &items[1] {
                                let mut db = db.lock().unwrap();
                                let removed = db.remove(key).is_some();
                                Some(format!(":{}\r\n", if removed { 1 } else { 0 }))
                            } else {
                                Some("-ERR invalid key\r\n".to_string())
                            }
                        }

                        _ => Some("-ERR unknown command\r\n".to_string()),
                    }
                }
                _ => Some("-ERR invalid command format\r\n".to_string()),
            }
        }
        _ => Some("-ERR expected array of command\r\n".to_string()),
    }
}
