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
    pub fn set(&self, header: SetOfHeaders, value: String) -> Self {
        self.headers.insert(header, value);
        *self
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        self.headers
            .iter()
            .fold("", |acc, (header, value)| {
                let head_name: String = match *header {
                    SetOfHeaders::AcceptRanges => "Accept-Ranges".to_string(),
                    SetOfHeaders::Age => "Age".to_string(),
                    SetOfHeaders::Allow => "Allow".to_string(),
                    SetOfHeaders::CacheControl => "Cache-Control".to_string(),
                    SetOfHeaders::Connection => "Connection".to_string(),
                    SetOfHeaders::ContentEncoding => "Content-Encoding".to_string(),
                    SetOfHeaders::ContentLanguage => "Content-Language".to_string(),
                    SetOfHeaders::ContentLength => "Content-Length".to_string(),
                    SetOfHeaders::ContentLocation => "Content-Location".to_string(),
                    SetOfHeaders::ContentSecurityPolicy => "Content-Security-Policy".to_string(),
                    SetOfHeaders::ContentType => "Content-Type".to_string(),
                    SetOfHeaders::Date => "Date".to_string(),
                    SetOfHeaders::ETag => "ETag".to_string(),
                    SetOfHeaders::Expires => "Expires".to_string(),
                    SetOfHeaders::LastModified => "Last-Modified".to_string(),
                    SetOfHeaders::Location => "Location".to_string(),
                    SetOfHeaders::ProxyAuthenticate => "Proxy-Authenticate".to_string(),
                    SetOfHeaders::RetryAfter => "Retry-After".to_string(),
                    SetOfHeaders::Server => "Server".to_string(),
                    SetOfHeaders::SetCookie => "Set-Cookie".to_string(),
                    SetOfHeaders::TransferEncoding => "Transfer-Encoding".to_string(),
                    SetOfHeaders::Vary => "Vary".to_string(),
                    SetOfHeaders::Via => "Via".to_string(),
                };

                &format!("{}{}{}{}: {}", acc, CR, LF, head_name, value)
            })
            .trim()
            .to_string()
    }
}
