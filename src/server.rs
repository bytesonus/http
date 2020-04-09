use crate::http_config::MiddlewareType;
use crate::http_config::HttpConfig;
use crate::http_config::MiddlewareContext;

use std::collections::HashMap;

use juno::models::Value;
use nickel::{Nickel, HttpRouter};

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
	let mut http_listener = crate::LISTENER.lock().unwrap();
	let http_config = crate::HTTP_CONFIG.lock().unwrap();

	if http_listener.is_some() {
		// Kill the server.
		// So far, no way to do that. Nickel doesn't support it yet
		panic!("Excuse me wtf?");
	}

	let mut server = Nickel::with_data(MiddlewareContext {
		data: Value::Null
	});

	for config in http_config.iter() {
		match &config.config_type {
			MiddlewareType::Delete => {
				server.delete::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Get => {
				server.get::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Options => {
				server.options::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Patch => {
				server.patch::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Post => {
				server.post::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Put => {
				server.put::<String, HttpConfig>(config.path.clone(), config.clone());
			},
			MiddlewareType::Use => {
				server.utilize::<HttpConfig>(config.clone());
			},
		}
	}

	// Populate `server` from HTTP_CONFIG
	// Setup routers and middlwares accordingly
	if args.get("socket").is_none() {
		// Need a binding address to bind to
		return Value::Null;
	}

	let listener = server
		.listen(args.get("socket").unwrap().as_string().unwrap())
		.unwrap();
	// Add `listener` to a global mutex like HTTP_CONFIG
	// Then, on clearConfig, if the global mutex is not null, force quit the server and start a new one
	*http_listener = Some(listener);

	Value::Null
}

pub fn clear_config(_args: HashMap<String, Value>) -> Value {
	crate::HTTP_CONFIG.lock().unwrap().clear();
	Value::Null
}