use crate::resp::RespType;

pub fn handle_command(resp: RespType) -> Option<String> {
    match resp {
        RespType::Array(Some(items)) => {
            if items.is_empty() {
                return Some("-ERR empty command\r\n".to_string());
            }
            match &items[0] {
                RespType::BulkString(Some(cmd)) => {
                    let cmd = cmd.to_uppercase();
                    match cmd.as_str() {
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
                        _ => Some("-ERR unknown command\r\n".to_string()),
                    }
                }
                _ => Some("-ERR invalid command format\r\n".to_string()),
            }
        }
        _ => Some("-ERR expected array of command\r\n".to_string()),
    }
}
