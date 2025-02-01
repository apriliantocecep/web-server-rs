use std::collections::HashMap;
use crate::http::HttpMethod;

#[derive(Debug)]
pub struct Request {
    pub http_method: HttpMethod,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}