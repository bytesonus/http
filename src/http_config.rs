#[derive(Clone)]
pub struct HttpConfig {
	pub config_type: MiddlewareType,
	pub func_name: String,
	pub path: String,
}

#[derive(Clone)]
pub enum MiddlewareType {
	Delete,
	Get,
	Options,
	Patch,
	Post,
	Put,
	Use,
}

impl MiddlewareType {
	pub fn to_capitalized_string(&self) -> &str {
		match self {
			MiddlewareType::Delete => "DELETE",
			MiddlewareType::Get => "GET",
			MiddlewareType::Options => "OPTIONS",
			MiddlewareType::Patch => "PATCH",
			MiddlewareType::Post => "POST",
			MiddlewareType::Put => "PUT",
			MiddlewareType::Use => "USE",
		}
	}
}
