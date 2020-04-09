use nickel::Middleware;
use gotham::models::Value;

pub struct HttpConfig {
    pub config_type: MiddlewareType,
    pub func_name: String,
    pub path: String
}

pub enum MiddlewareType {
    USE,
    DELETE,
    GET,
    OPTIONS,
    POST,
    PUT,
    UPDATE,
}

pub struct MiddlewareContext {
    pub data: Value
}

impl Middleware<MiddlewareContext> for HttpConfig {
    // TODO implement the trait
}
