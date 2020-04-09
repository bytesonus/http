use gotham::models::Value;
use nickel::{hyper::net::Fresh, Middleware, MiddlewareResult, Request, Response};

pub struct HttpConfig {
	pub config_type: MiddlewareType,
	pub func_name: String,
	pub path: String,
}

pub enum MiddlewareType {
	DELETE,
	GET,
	OPTIONS,
	POST,
	PUT,
	UPDATE,
	USE,
}

pub struct MiddlewareContext {
	pub data: Value,
}

impl Middleware<MiddlewareContext> for HttpConfig {
	fn invoke<'mw, 'conn>(
		&'mw self,
		_req: &mut Request<'mw, 'conn, MiddlewareContext>,
		res: Response<'mw, MiddlewareContext, Fresh>,
	) -> MiddlewareResult<'mw, MiddlewareContext> {
		res.next_middleware()
	}
}
