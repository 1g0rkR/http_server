use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use crate::application::http::{
    http_consts::{SEPARATOR_BODY, SIZE},
    http_headers::{Headers, SetOfHeaders},
    http_response::{Response, Response200, Response400, Response404, ResponseCode},
};

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
                    let path = Path::new(self.directory.as_str()).join(filename);
                    if path.exists() {
                        let headers = Headers::new()
                            .set(SetOfHeaders::Server, "Rust Server")
                            .set(SetOfHeaders::ContentType, "application/octet-stream")
                            .set(SetOfHeaders::TransferEncoding, "chunked");

                        let mut buffer: [u8; SIZE] = [0; SIZE];
                        let mut file = fs::File::open(path).unwrap();
                        stream
                            .write(
                                Response::new(ResponseCode::Ok, headers)
                                    .to_string()
                                    .as_bytes(),
                            )
                            .unwrap();
                        stream.write(SEPARATOR_BODY.as_bytes()).unwrap();
                        // stream.write(b"HTTP/1.1 200 OK\r\nServer: Server\r\nContent-Type: application/octet-stream\r\nTransfer-Encoding: chunked\r\n\r\n").unwrap();
                        stream.flush().unwrap();
                        while let Ok(bytes) = file.read(&mut buffer) {
                            if bytes == 0 {
                                break;
                            } else {
                                stream.write(format!("{:x}\r\n", bytes).as_bytes()).unwrap();
                                stream.write(&buffer[..bytes]).unwrap();
                                stream.write(b"\r\n").unwrap();

                                if bytes < SIZE {
                                    stream.write(b"0").unwrap();
                                    stream.write(SEPARATOR_BODY.as_bytes()).unwrap();
                                    // stream.write(b"0\r\n\r\n").unwrap();
                                    stream.flush().unwrap();
                                    break;
                                }
                                stream.flush().unwrap();
                            }
                        }
                    } else {
                        stream.write(Response404::get().as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                } else {
                    stream.write(Response400::get().as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            } else if base.to_lowercase() == "list" {
                let mut dir = fs::read_dir(self.directory.as_str()).unwrap();
                let mut buffer: String = String::new();
                while let Some(file) = dir.next() {
                    let a = file.unwrap();
                    buffer.push_str(a.file_name().to_str().unwrap());
                    buffer.push_str("\r\n");
                }
                stream.write(Response200::get(buffer).as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                stream.write(Response404::get().as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else {
            stream.write(Response404::get().as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        ()
    }
}
