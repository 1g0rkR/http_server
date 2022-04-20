use crate::errors::HttpErrors;
use std::collections::HashMap;

const SPACE_SYMBOL: &'static str = " ";
const COLON_SYMBOL: &'static str = ":";
const SEPARATOR_BODY: &'static str = "\r\n\r\n";

pub fn parse_start_line(buffer: &str) -> Result<(String, String, String), HttpErrors> {
    let start_line: &str;
    if let Some(first) = buffer.lines().next() {
        start_line = first;
    } else {
        return Err(HttpErrors::ErrorParseStartLine);
    }
    let mut parts = start_line.split(SPACE_SYMBOL);

    match (parts.next(), parts.next(), parts.next()) {
        (Some(method), Some(path), Some(version)) => {
            Ok((method.to_string(), path.to_string(), version.to_string()))
        }
        _ => Err(HttpErrors::ErrorParseStartLine),
    }
}

pub fn parse_headers(buffer: &str) -> Result<HashMap<String, String>, HttpErrors> {
    let mut headers: HashMap<String, String> = HashMap::new();

    let mut iterator = buffer.lines();
    iterator.next();

    while let Some(header) = iterator.next() {
        if header.len() == 0 {
            break;
        } else {
            if let Some((name, value)) = header.split_once(COLON_SYMBOL) {
                headers.insert(name.trim().to_string(), value.trim().to_string());
            }
        }
    }

    Ok(headers)
}

pub fn get_body(buffer: &str) -> Result<String, HttpErrors> {
    if let Some(index) = buffer.find(SEPARATOR_BODY) {
        let (_, body) = buffer.split_at(index);
        Ok(body.trim().to_string())
    } else {
        Err(HttpErrors::ErrorGetBody)
    }
}
