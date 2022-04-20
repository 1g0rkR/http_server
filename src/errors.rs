#[derive(Debug)]
pub enum HttpErrors {
    ErrorParseStartLine,
    ErrorParseHeaders,
    ErrorGetBody,
}
