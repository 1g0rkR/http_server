use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use crate::application::http::http_consts::SIZE;

const DIRECTORY: &'static str = "./public";

pub struct GET {
    path: String,
    version: String,
    headers: HashMap<String, String>,
    directory: String,
}

impl GET {
    pub fn new(
        path: String,
        version: String,
        headers: HashMap<String, String>,
        directory: String,
    ) -> GET {
        GET {
            path,
            version,
            headers,
            directory,
        }
    }

    pub fn execute(&self, stream: &mut TcpStream) {
        let mut path_parts = self.path.split("/").skip(1);

        if let Some(base) = path_parts.next() {
            if base.to_lowercase() == "get" {
                if let Some(filename) = path_parts.next() {
                    let path = Path::new(DIRECTORY).join(filename);
                    if path.exists() {
                        let mut buffer: [u8; SIZE] = [0; SIZE];
                        let mut file = fs::File::open(path).unwrap();
                        stream.write(b"HTTP/1.1 200 OK\r\nServer: Server\r\nContent-Type: application/octet-stream\r\nTransfer-Encoding: chunked\r\n\r\n").unwrap();
                        stream.flush().unwrap();
                        while let Ok(bytes) = file.read(&mut buffer) {
                            if bytes == 0 {
                                break;
                            } else {
                                stream.write(format!("{:x}\r\n", bytes).as_bytes()).unwrap();
                                stream.write(&buffer[..bytes]).unwrap();
                                stream.write(b"\r\n").unwrap();

                                if bytes < SIZE {
                                    stream.write(b"0\r\n\r\n").unwrap();
                                    stream.flush().unwrap();
                                    break;
                                }
                                stream.flush().unwrap();
                            }
                        }
                    } else {
                        stream
                            .write(b"HTTP/1.1 404 NOT FOUND\r\nServer: Server")
                            .unwrap();
                        stream.flush().unwrap();
                    }
                } else {
                    stream
                        .write(b"HTTP/1.1 400 BAD REQUEST\r\nServer: Server")
                        .unwrap();
                    stream.flush().unwrap();
                }
            } else if base.to_lowercase() == "list" {
                let mut dir = fs::read_dir(DIRECTORY).unwrap();
                let mut buffer: String = String::new();
                while let Some(file) = dir.next() {
                    let a = file.unwrap();
                    buffer.push_str(a.file_name().to_str().unwrap());
                    buffer.push_str("\r\n");
                }
                let wrt=format!("HTTP/1.1 200 OK\r\nServer: Server\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",buffer.len(),&buffer);

                stream.write(wrt.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                stream
                    .write(b"HTTP/1.1 404 NOT FOUND\r\nServer: Server")
                    .unwrap();
                stream.flush().unwrap();
            }
        } else {
            stream
                .write(b"HTTP/1.1 404 NOT FOUND\r\nServer: Server")
                .unwrap();
            stream.flush().unwrap();
        }

        ()
    }
}
