use std::net::TcpStream;
pub mod http;

pub struct Application {}

impl Application {
    pub fn get(stream: TcpStream) {
        http::HttpRequest::new(stream).execute();
    }
}
