use crate::http_config::MiddlewareType;
use crate::http_config::HttpConfig;

use std::collections::HashMap;

use gotham::models::Value;
use nickel::Nickel;

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
		config_type: MiddlewareType::DELETE,
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
		config_type: MiddlewareType::GET,
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
		config_type: MiddlewareType::OPTIONS,
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
		config_type: MiddlewareType::POST,
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
		config_type: MiddlewareType::PUT,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn app_update(args: HashMap<String, Value>) -> Value {
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
		config_type: MiddlewareType::UPDATE,
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
		config_type: MiddlewareType::USE,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

pub fn listen(args: HashMap<String, Value>) -> Value {
	let mut server = Nickel::new();
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

	Value::Null
}

pub fn clear_config(_args: HashMap<String, Value>) -> Value {
    crate::HTTP_CONFIG.lock().unwrap().clear();
	Value::Null
}