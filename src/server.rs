use crate::{
	http_config::{HttpConfig, MiddlewareType},
	http_context::HttpContext,
};

use std::collections::HashMap;

use juno::models::Number;
use juno::models::Value;
use thruster::{
	async_middleware, middleware_fn, App, Context, MiddlewareNext, MiddlewareResult, Request,
	Server, ThrusterServer,
};

pub fn app_delete(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Delete,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_get(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Get,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_options(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Options,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_patch(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Patch,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_post(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Post,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_put(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Put,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_use(args: HashMap<String, Value>) -> Value {
	let value = Value::String(String::from("/"));
	let string = String::from("/");
	let path = args
		.get("path")
		.unwrap_or(&value)
		.as_string()
		.unwrap_or(&string);

	let function = args.get("function").unwrap_or(&Value::Null).as_string();
	if function.is_none() {
		return Value::Null;
	}
	let function = function.unwrap();

	crate::HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::Use,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn listen(args: HashMap<String, Value>) -> Value {
	let http_listener = crate::LISTENER.lock().unwrap();
	let http_config = crate::HTTP_CONFIG.lock().unwrap();

	if http_listener.is_some() {
		// Kill the server.
		// So far, no way to do that. Nickel doesn't support it yet
		panic!("Excuse me wtf?");
	}

	let mut app =
		App::<Request, HttpContext, Vec<HttpConfig>>::create(generate_context, http_config.clone());
	app.use_middleware("/", async_middleware!(HttpContext, [handle_middleware]));

	for config in http_config.iter() {
		let path = config.path.clone();
		let path = Box::leak(path.into_boxed_str());
		match &config.config_type {
			MiddlewareType::Delete => {
				app.delete(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Get => {
				app.get(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Options => {
				app.options(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Patch => {
				app.update(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Post => {
				app.post(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Put => {
				app.put(path, async_middleware!(HttpContext, [handle_data]));
			}
			MiddlewareType::Use => {}
		}
	}
	app.set404(async_middleware!(HttpContext, [handle_404]));

	let server = Server::new(app);

	// Populate `server` from HTTP_CONFIG
	// Setup routers and middlwares accordingly
	if args.get("socket").is_none() {
		// Need a binding address to bind to
		return Value::Null;
	}

	tokio::spawn(async move {
		server
			.build(
				args.get("socket")
					.unwrap_or(&Value::String("127.0.0.1".to_string()))
					.as_string()
					.unwrap_or(&String::from("127.0.0.1")),
				args.get("port")
					.unwrap_or(&Value::Number(Number::PosInt(3000)))
					.as_number()
					.unwrap_or(&Number::PosInt(3000))
					.as_u64()
					.unwrap_or(3000) as u16,
			)
			.await;
	});
	// Add `listener` to a global mutex like HTTP_CONFIG
	// Then, on clearConfig, if the global mutex is not null, force quit the server and start a new one
	//*http_listener = Some(server);

	Value::Null
}

pub fn clear_config(_args: HashMap<String, Value>) -> Value {
	crate::HTTP_CONFIG.lock().unwrap().clear();
	Value::Null
}

fn generate_context(request: Request, state: &Vec<HttpConfig>, path: &str) -> HttpContext {
	let start = request.method().len() + 4;
	let matched_route = String::from(&path[start..]);

	HttpContext::new(request, matched_route, state.clone())
}

#[middleware_fn]
async fn handle_data(
	mut context: HttpContext,
	next: MiddlewareNext<HttpContext>,
) -> MiddlewareResult<HttpContext> {
	let fn_name = context
		.route_configs
		.iter()
		.filter(|config| {
			context.method() == config.config_type.to_capitalized_string()
				&& context.matched_route() == &config.path
		})
		.next();
	if fn_name.is_none() {
		return next(context).await;
	}
	let fn_name = fn_name.unwrap();
	// TODO get an instance to juno module from here and call the function

	Ok(context)
}

#[middleware_fn]
async fn handle_middleware(
	context: HttpContext,
	next: MiddlewareNext<HttpContext>,
) -> MiddlewareResult<HttpContext> {
	println!("Got middleware on {}", context.matched_route());
	next(context).await
}

#[middleware_fn]
async fn handle_404(
	mut context: HttpContext,
	_next: MiddlewareNext<HttpContext>,
) -> MiddlewareResult<HttpContext> {
	context.status(404);
	context.body(&format!("Cannot {} {}", context.method(), context.route()));
	Ok(context)
}
