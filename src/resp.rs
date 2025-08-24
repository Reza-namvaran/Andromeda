use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Option<Vec<RespType>>),
}

pub fn parse_resp<R: BufRead>(reader: &mut R) -> io::Result<RespType> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let prefix = line.chars().next().ok_or_else(|| { io::Error::new(io::ErrorKind::InvalidData, "Empty Input")})?;

    match prefix {
        '+' => Ok(RespType::SimpleString(line[1..].trim_end().to_string())),
        '-' => Ok(RespType::Error(line[1..].trim_end().to_string)),
        ':' => {
            let num = line[1..].trim_end().parse().map_err(|_| { io::Error::new(io::ErrorKind::InvalidData, "Invalid int")})?;
            Ok(RespType::Integer(num))
        }
        '$' => {
            let len: isize = line[1..].trim_end().parse().map_err(|_| { io::Error::new(io::ErrorKind::InvalidData, "Invalid bulk string length")})?;
            if len == -1 {
                return Ok(RespType::BulkString(None));
            }
            
            let mut buf = vec![0; len as usize + 2]; // include CRLF
            reader.read_exact(&mut buf)?;
            let s = String::from_utf8_lossy(&buf[..len as usize]).to_string();
            Ok(RespType::BulkString(Some(s)))
        }
        '*' => {
            let count: isize = line[..1].trim_end().parse().map_err(|_| { io::Error::new(io::ErrorKind::InvalidData, "Invalid array length")})?;
            if count == -1 {
                return Ok(RespType::Array(None));
            }

            let mut items = Vec::new();
            for _ in 0..count {
                items.push(parse_resp(reader)?);
            }
            Ok(RespType::Array(Some(items)))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown RESP type")),
    }
}