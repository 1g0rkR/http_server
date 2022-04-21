use std::{fmt, time::SystemTime};

use super::{
    http_consts::HTTP_VERSION,
    http_consts::{CR, LF},
    http_headers::{Headers, SetOfHeaders},
};

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
    headers: Headers,
}

impl Response {
    pub fn new(code: ResponseCode, headers: Headers) -> Response {
        Response { code, headers }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let headers_str = self.headers.to_string();
        match self.code {
            ResponseCode::Continue => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Continue,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::SwitchingProtocols => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::SwitchingProtocols,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Processing => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Processing,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Ok => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Ok,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Created => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Created,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Accepted => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Accepted,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::NoContent => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::NoContent,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::ResetContent => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::ResetContent,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::PartialContent => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::PartialContent,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::MultiStatus => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::MultiStatus,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::IMUsed => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::IMUsed,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::MovedPermanently => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::MovedPermanently,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Found => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Found,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::SeeOther => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::SeeOther,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::NotModified => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::NotModified,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::UseProxy => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::UseProxy,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::TemporaryRedirect => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::TemporaryRedirect,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::BadRequest => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::BadRequest,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Unauthorized => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Unauthorized,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::PaymentRequired => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::PaymentRequired,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::Forbidden => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::Forbidden,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::NotFound => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::NotFound,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::RequestTimeout => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::RequestTimeout,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::InternalServerError => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::InternalServerError,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::BadGateway => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::BadGateway,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::ServiceUnavailable => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::ServiceUnavailable,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::GatewayTimeout => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::GatewayTimeout,
                    CR,
                    LF,
                    headers_str
                )
            }
            ResponseCode::HTTPVersionNotSupported => {
                format!(
                    "{} {}{}{}{}",
                    HTTP_VERSION,
                    ResponseCode::HTTPVersionNotSupported,
                    CR,
                    LF,
                    headers_str
                )
            }
        }
    }
}

pub struct Response404 {}

impl Response404 {
    pub fn get() -> String {
        let text = "ERROR 404 NOT FOUND";
        let line = format!("{}{}{}{}", CR, LF, CR, LF);
        let headers = Headers::new()
            .set(SetOfHeaders::Server, "Rust Server")
            .set(SetOfHeaders::ContentType, "text/plain")
            .set(SetOfHeaders::ContentLength, &format!("{}", text.len()));

        format!(
            "{}{}{}{}",
            Response::new(ResponseCode::NotFound, headers).to_string(),
            &line,
            &text,
            &line
        )
    }
}

pub struct Response400 {}

impl Response400 {
    pub fn get() -> String {
        let text = "ERROR 400 BAD REQUEST";
        let line = format!("{}{}{}{}", CR, LF, CR, LF);
        let headers = Headers::new()
            .set(SetOfHeaders::Server, "Rust Server")
            .set(SetOfHeaders::ContentType, "text/plain")
            .set(SetOfHeaders::ContentLength, &format!("{}", text.len()));

        format!(
            "{}{}{}{}",
            Response::new(ResponseCode::BadRequest, headers).to_string(),
            &line,
            &text,
            &line
        )
    }
}

pub struct Response200 {}

impl Response200 {
    pub fn get(body: String) -> String {
        let line = format!("{}{}{}{}", CR, LF, CR, LF);
        let headers = Headers::new()
            .set(SetOfHeaders::Server, "Rust Server")
            .set(SetOfHeaders::ContentType, "text/plain")
            .set(SetOfHeaders::ContentLength, &format!("{}", body.len()));

        format!(
            "{}{}{}{}",
            Response::new(ResponseCode::Ok, headers).to_string(),
            &line,
            &body,
            &line
        )
    }

    pub fn get_with_headers(body: String, headers: Headers) -> String {
        let line = format!("{}{}{}{}", CR, LF, CR, LF);

        format!(
            "{}{}{}{}",
            Response::new(ResponseCode::Ok, headers).to_string(),
            &line,
            &body,
            &line
        )
    }
}
