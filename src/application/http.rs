mod http_consts;
mod http_headers;
mod http_methods;
mod http_parse;
pub mod http_response;

use http_parse::{get_body, parse_headers, parse_start_line};
use std::io::Write;
use std::{collections::HashMap, io::Read, net::TcpStream};

use self::http_consts::SIZE;
use self::http_methods::method_get;
use self::http_response::{Response400, Response501};

pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
    stream: TcpStream,
}

impl HttpRequest {
    pub fn new(mut stream: TcpStream) -> HttpRequest {
        let mut buffer: String = String::new();
        let mut buf = [0u8; SIZE];

        while let Ok(count) = stream.read(&mut buf[..]) {
            if count == 0 {
                break;
            } else {
                buffer.push_str(&String::from_utf8_lossy(&buf[..count]).to_string());
                if count < SIZE {
                    break;
                }
            }
        }
        let mut method = String::new();
        let mut path = String::new();
        let mut version = String::new();
        ("".to_string(), "".to_string(), "".to_string());
        if let Ok((_method, _path, _version)) = parse_start_line(&buffer) {
            method = _method;
            path = _path;
            version = _version;
        } else {
            stream.write(Response400::get().as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(hashmap) = parse_headers(&buffer) {
            headers = hashmap;
        } else {
            stream.write(Response400::get().as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        if let Ok(body) = get_body(&buffer) {
            return HttpRequest {
                method,
                path,
                version,
                headers,
                body,
                stream,
            };
        }

        HttpRequest {
            method,
            path,
            version,
            headers,
            body: "".to_string(),
            stream,
        }
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn execute(&mut self) {
        match self.get_method() {
            "GET" => {
                method_get::GET::new(
                    self.path.clone(),
                    self.version.clone(),
                    self.headers.clone(),
                    "./public".to_string(),
                )
                .execute(&mut self.stream);
            }
            _ => {
                self.stream.write(Response501::get().as_bytes()).unwrap();
                self.stream.flush().unwrap();
            }
        }
    }
}
