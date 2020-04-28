use juno::{models::Value, Result};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::oneshot::Sender;

pub enum HttpServerCommands {
	AddConfig(HttpConfig),
	ClearConfig,
	Listen(SocketAddr),
}

pub enum JunoCommands {
	CallFunction(String, HashMap<String, Value>, Sender<Result<Value>>),
	TriggerHook(String),
}

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
