mod http_consts;
mod http_headers;
mod http_methods;
mod http_parse;
mod http_response;

use crate::errors::HttpErrors;
use http_parse::{get_body, parse_headers, parse_start_line};
use std::{collections::HashMap, io::Read, net::TcpStream};

use self::http_consts::SIZE;
use self::http_methods::method_get;

pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
    stream: TcpStream,
}

impl HttpRequest {
    pub fn new(mut stream: TcpStream) -> Result<HttpRequest, HttpErrors> {
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
        let (method, path, version) = parse_start_line(&buffer)?;

        let headers: HashMap<String, String> = parse_headers(&buffer)?;

        if let Ok(body) = get_body(&buffer) {
            return Ok(HttpRequest {
                method,
                path,
                version,
                headers,
                body,
                stream,
            });
        }

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body: "".to_string(),
            stream,
        })
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
            _ => panic!("Panic"),
        }
    }
}
