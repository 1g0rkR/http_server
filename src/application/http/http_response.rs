use std::fmt;

use crate::application::http::http_consts::HTTP_VERSION;

pub enum ResponseCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    IMUsed = 226,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    RequestTimeout = 408,
    InternalServerError = 500,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match *self {
            ResponseCode::Continue => "100 Continue",
            ResponseCode::SwitchingProtocols => "101 Switching Protocols",
            ResponseCode::Processing => "102 Processing",
            ResponseCode::Ok => "200 OK",
            ResponseCode::Created => "201 Created",
            ResponseCode::Accepted => "202 Accepted",
            ResponseCode::NoContent => "204 No Content",
            ResponseCode::ResetContent => "205 Reset Content",
            ResponseCode::PartialContent => "206 Partial Content",
            ResponseCode::MultiStatus => "207 Multi-Status",
            ResponseCode::IMUsed => "226 IM Used",
            ResponseCode::MovedPermanently => "301 Moved Permanently",
            ResponseCode::Found => "302 Found",
            ResponseCode::SeeOther => "303 See Other",
            ResponseCode::NotModified => "304 Not Modified",
            ResponseCode::UseProxy => "305 Use Proxy",
            ResponseCode::TemporaryRedirect => "307 Temporary Redirect",
            ResponseCode::BadRequest => "400 Bad Request",
            ResponseCode::Unauthorized => "401 Unauthorized",
            ResponseCode::PaymentRequired => "402 Payment Required",
            ResponseCode::Forbidden => "403 Forbidden",
            ResponseCode::NotFound => "404 Not Found",
            ResponseCode::RequestTimeout => "408 Request Timeout",
            ResponseCode::InternalServerError => "500 Internal Server Error",
            ResponseCode::BadGateway => "502 Bad Gateway",
            ResponseCode::ServiceUnavailable => "503 Service Unavailable",
            ResponseCode::GatewayTimeout => "504 Gateway Timeout",
            ResponseCode::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
        };
        write!(f, "{}", output)
    }
}

pub struct Response {
    code: ResponseCode,
}

impl Response {
    pub fn new(code: ResponseCode) -> Response {
        Response { code }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        match self.code {
            ResponseCode::Continue => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Continue)
            }
            ResponseCode::SwitchingProtocols => {
                format!("{} {}", HTTP_VERSION, ResponseCode::SwitchingProtocols)
            }
            ResponseCode::Processing => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Processing)
            }
            ResponseCode::Ok => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Ok)
            }
            ResponseCode::Created => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Created)
            }
            ResponseCode::Accepted => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Accepted)
            }
            ResponseCode::NoContent => {
                format!("{} {}", HTTP_VERSION, ResponseCode::NoContent)
            }
            ResponseCode::ResetContent => {
                format!("{} {}", HTTP_VERSION, ResponseCode::ResetContent)
            }
            ResponseCode::PartialContent => {
                format!("{} {}", HTTP_VERSION, ResponseCode::PartialContent)
            }
            ResponseCode::MultiStatus => {
                format!("{} {}", HTTP_VERSION, ResponseCode::MultiStatus)
            }
            ResponseCode::IMUsed => {
                format!("{} {}", HTTP_VERSION, ResponseCode::IMUsed)
            }
            ResponseCode::MovedPermanently => {
                format!("{} {}", HTTP_VERSION, ResponseCode::MovedPermanently)
            }
            ResponseCode::Found => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Found)
            }
            ResponseCode::SeeOther => {
                format!("{} {}", HTTP_VERSION, ResponseCode::SeeOther)
            }
            ResponseCode::NotModified => {
                format!("{} {}", HTTP_VERSION, ResponseCode::NotModified)
            }
            ResponseCode::UseProxy => {
                format!("{} {}", HTTP_VERSION, ResponseCode::UseProxy)
            }
            ResponseCode::TemporaryRedirect => {
                format!("{} {}", HTTP_VERSION, ResponseCode::TemporaryRedirect)
            }
            ResponseCode::BadRequest => {
                format!("{} {}", HTTP_VERSION, ResponseCode::BadRequest)
            }
            ResponseCode::Unauthorized => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Unauthorized)
            }
            ResponseCode::PaymentRequired => {
                format!("{} {}", HTTP_VERSION, ResponseCode::PaymentRequired)
            }
            ResponseCode::Forbidden => {
                format!("{} {}", HTTP_VERSION, ResponseCode::Forbidden)
            }
            ResponseCode::NotFound => {
                format!("{} {}", HTTP_VERSION, ResponseCode::NotFound)
            }
            ResponseCode::RequestTimeout => {
                format!("{} {}", HTTP_VERSION, ResponseCode::RequestTimeout)
            }
            ResponseCode::InternalServerError => {
                format!("{} {}", HTTP_VERSION, ResponseCode::InternalServerError)
            }
            ResponseCode::BadGateway => {
                format!("{} {}", HTTP_VERSION, ResponseCode::BadGateway)
            }
            ResponseCode::ServiceUnavailable => {
                format!("{} {}", HTTP_VERSION, ResponseCode::ServiceUnavailable)
            }
            ResponseCode::GatewayTimeout => {
                format!("{} {}", HTTP_VERSION, ResponseCode::GatewayTimeout)
            }
            ResponseCode::HTTPVersionNotSupported => {
                format!("{} {}", HTTP_VERSION, ResponseCode::HTTPVersionNotSupported)
            }
        }
    }
}
