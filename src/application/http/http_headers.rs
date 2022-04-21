use crate::application::http::http_consts::{CR, LF};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum SetOfHeaders {
    AcceptRanges,
    Age,
    Allow,
    CacheControl,
    Connection,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentSecurityPolicy,
    ContentType,
    Date,
    ETag,
    Expires,
    LastModified,
    Location,
    ProxyAuthenticate,
    RetryAfter,
    Server,
    SetCookie,
    TransferEncoding,
    Vary,
    Via,
}

pub struct Headers {
    headers: HashMap<SetOfHeaders, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new(),
        }
    }
    pub fn set(mut self, header: SetOfHeaders, value: &str) -> Self {
        self.headers.insert(header, value.to_string());
        self
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        self.headers
            .iter()
            .fold("".to_string(), |acc, (header, value)| {
                let head_name: &str = match *header {
                    SetOfHeaders::AcceptRanges => "Accept-Ranges",
                    SetOfHeaders::Age => "Age",
                    SetOfHeaders::Allow => "Allow",
                    SetOfHeaders::CacheControl => "Cache-Control",
                    SetOfHeaders::Connection => "Connection",
                    SetOfHeaders::ContentEncoding => "Content-Encoding",
                    SetOfHeaders::ContentLanguage => "Content-Language",
                    SetOfHeaders::ContentLength => "Content-Length",
                    SetOfHeaders::ContentLocation => "Content-Location",
                    SetOfHeaders::ContentSecurityPolicy => "Content-Security-Policy",
                    SetOfHeaders::ContentType => "Content-Type",
                    SetOfHeaders::Date => "Date",
                    SetOfHeaders::ETag => "ETag",
                    SetOfHeaders::Expires => "Expires",
                    SetOfHeaders::LastModified => "Last-Modified",
                    SetOfHeaders::Location => "Location",
                    SetOfHeaders::ProxyAuthenticate => "Proxy-Authenticate",
                    SetOfHeaders::RetryAfter => "Retry-After",
                    SetOfHeaders::Server => "Server",
                    SetOfHeaders::SetCookie => "Set-Cookie",
                    SetOfHeaders::TransferEncoding => "Transfer-Encoding",
                    SetOfHeaders::Vary => "Vary",
                    SetOfHeaders::Via => "Via",
                };

                format!("{}{}{}{}: {}", acc, CR, LF, head_name, value)
            })
            .trim()
            .to_string()
    }
}
