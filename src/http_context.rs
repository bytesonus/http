use crate::{misc::JunoCommands, HttpConfig};
use bytes::Bytes;
use juno::models::Value;
use serde_json::from_str;
use std::{collections::HashMap, str};
use thruster::{
	middleware::{
		cookies::{Cookie, CookieOptions, HasCookies, SameSite},
		query_params::HasQueryParams,
	},
	Context, Request, Response,
};
use tokio::sync::mpsc::UnboundedSender;

pub struct HttpContext {
	response: Response,
	cookies: Vec<Cookie>,
	params: HashMap<String, String>,
	query_params: HashMap<String, String>,
	request: Request,
	status: u32,
	headers: HashMap<String, String>,
	matched_route: String,
	http_version: u8,
	juno_representation: HashMap<String, Value>,
	pub http_configs: Vec<HttpConfig>,
	pub juno_sender: UnboundedSender<JunoCommands>,
	pub data: Value,
}

impl HttpContext {
	pub fn new(
		request: Request,
		matched_route: String,
		http_configs: Vec<HttpConfig>,
		juno_sender: UnboundedSender<JunoCommands>,
	) -> Self {
		let body = request.body().to_string();
		let http_version = request.version();
		let mut context = HttpContext {
			response: Response::new(),
			cookies: Vec::new(),
			params: request.params().clone(),
			query_params: HashMap::new(),
			headers: request.headers(),
			request,
			status: 200,
			matched_route,
			http_version,
			juno_representation: HashMap::new(),
			http_configs,
			juno_sender,
			data: Value::Null,
		};
		context.body(&body);
		context.set("Server", "Thruster");
		context.generate_juno_representation();

		context
	}

	pub fn matched_route(&self) -> &String {
		&self.matched_route
	}

	pub fn body(&mut self, body_string: &str) {
		self.response
			.body_bytes_from_vec(body_string.as_bytes().to_vec());
	}
	pub fn get_body(&self) -> String {
		str::from_utf8(&self.response.response)
			.unwrap_or("")
			.to_owned()
	}

	pub fn status(&mut self, code: u32) {
		self.status = code;
	}

	pub fn set_content_type(&mut self, c_type: &str) {
		self.set("Content-Type", c_type);

		if c_type == "application/json" {
			let body: serde_json::Value =
				from_str(&self.get_body()).unwrap_or(serde_json::Value::Null);
			self.juno_representation
				.insert(String::from("body"), From::from(body));
		} else {
			self.juno_representation
				.insert(String::from("body"), Value::String(self.get_body()));
		}
	}

	pub fn redirect(&mut self, destination: &str) {
		self.status(302);

		self.set("Location", destination);
	}

	pub fn method(&self) -> &str {
		&self.request.method()
	}

	pub fn http_version(&self) -> u8 {
		self.http_version
	}

	pub fn juno_representation(&self) -> &HashMap<String, Value> {
		&self.juno_representation
	}

	pub fn generate_juno_representation(&mut self) {
		let mut map = HashMap::new();

		map.insert(
			String::from("httpVersion"),
			Value::String(format!("{}", self.http_version())),
		);

		map.insert(
			String::from("headers"),
			Value::Object(
				self.headers
					.iter()
					.map(|item| (item.0.clone(), Value::String(item.1.clone())))
					.collect(),
			),
		);

		let mut raw_headers: Vec<Value> = vec![];
		self.headers.iter().for_each(|item| {
			raw_headers.push(Value::String(item.0.clone()));
			raw_headers.push(Value::String(item.1.clone()));
		});
		map.insert(String::from("rawHeaders"), Value::Array(raw_headers));

		map.insert(String::from("url"), Value::String(self.route().to_string()));
		map.insert(
			String::from("method"),
			Value::String(self.method().to_string()),
		);

		map.insert(
			String::from("params"),
			Value::Object(
				self.params
					.iter()
					.map(|item| (item.0.clone(), Value::String(item.1.clone())))
					.collect(),
			),
		);

		map.insert(
			String::from("query"),
			Value::Object(
				self.query_params
					.iter()
					.map(|item| (item.0.clone(), Value::String(item.1.clone())))
					.collect(),
			),
		);

		map.insert(
			String::from("cookies"),
			Value::Object(
				self.cookies
					.iter()
					.map(|item| (item.key.clone(), Value::String(item.value.clone())))
					.collect(),
			),
		);

		match self.headers.get("Content-Type") {
			Some(value) if value == "application/json" => {
				let body: serde_json::Value =
					from_str(&self.get_body()).unwrap_or(serde_json::Value::Null);
				map.insert(String::from("body"), From::from(body));
			}
			_ => {
				map.insert(String::from("body"), Value::String(self.get_body()));
			}
		};

		self.juno_representation = map;
	}

	pub fn set_cookie(&mut self, name: &str, value: &str, options: &CookieOptions) {
		let cookie_value = match self.headers.get("Set-Cookie") {
			Some(val) => format!("{}, {}", val, self.cookify_options(name, value, &options)),
			None => self.cookify_options(name, value, &options),
		};

		self.set("Set-Cookie", &cookie_value);
	}

	fn cookify_options(&self, name: &str, value: &str, options: &CookieOptions) -> String {
		let mut pieces = vec![format!("Path={}", options.path)];

		if options.expires > 0 {
			pieces.push(format!("Expires={}", options.expires));
		}

		if options.max_age > 0 {
			pieces.push(format!("Max-Age={}", options.max_age));
		}

		if !options.domain.is_empty() {
			pieces.push(format!("Domain={}", options.domain));
		}

		if options.secure {
			pieces.push("Secure".to_owned());
		}

		if options.http_only {
			pieces.push("HttpOnly".to_owned());
		}

		if let Some(ref same_site) = options.same_site {
			match same_site {
				SameSite::Strict => pieces.push("SameSite=Strict".to_owned()),
				SameSite::Lax => pieces.push("SameSite=Lax".to_owned()),
			};
		}

		format!("{}={}; {}", name, value, pieces.join(", "))
	}
}

impl Context for HttpContext {
	type Response = Response;

	fn get_response(mut self) -> Self::Response {
		self.response.status_code(self.status, "");

		self.response
	}

	fn set_body(&mut self, body: Vec<u8>) {
		self.response.body_bytes_from_vec(body);
	}

	fn set_body_bytes(&mut self, body_bytes: Bytes) {
		self.response.body_bytes(&body_bytes);
	}

	fn route(&self) -> &str {
		self.request.path()
	}

	fn set(&mut self, key: &str, value: &str) {
		self.headers.insert(key.to_owned(), value.to_owned());
		if let Value::Object(map) = self.juno_representation.get_mut("headers").unwrap() {
			map.insert(key.to_string(), Value::String(value.to_string()));
		}
	}

	fn remove(&mut self, key: &str) {
		self.headers.remove(key);
	}
}

impl HasQueryParams for HttpContext {
	fn set_query_params(&mut self, query_params: HashMap<String, String>) {
		self.query_params = query_params;
	}
}

impl HasCookies for HttpContext {
	fn set_cookies(&mut self, cookies: Vec<Cookie>) {
		self.cookies = cookies;
	}

	fn headers(&self) -> HashMap<String, String> {
		self.request.headers()
	}
}
