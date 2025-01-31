use crate::http::HttpMethod;

pub struct Route {
    http_method: HttpMethod,
    path: String,
}