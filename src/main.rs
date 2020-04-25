#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate clap;
extern crate futures;
extern crate juno;
extern crate thruster;
extern crate tokio;

mod cli_parser;
mod http_config;
mod http_context;
mod server;

use http_config::HttpConfig;

use std::{collections::HashMap, sync::Mutex, time::Duration};

use thruster::Server;

use clap::{crate_name, crate_version};

lazy_static! {
	pub static ref HTTP_CONFIG: Mutex<Vec<HttpConfig>> = Mutex::new(Vec::new());
	pub static ref LISTENER: Mutex<Option<Server<http_context::HttpContext, ()>>> =
		Mutex::new(None);
}

//#[tokio::main]
fn main() {
	server::listen(HashMap::new());

	/*
	let mut module = cli_parser::from_cli_args();
	module
		.initialize(crate_name!(), crate_version!(), HashMap::new())
		.await
		.unwrap();

	module
		.declare_function("delete", server::app_delete)
		.await
		.unwrap();
	module
		.declare_function("get", server::app_get)
		.await
		.unwrap();
	module
		.declare_function("options", server::app_options)
		.await
		.unwrap();
	module
		.declare_function("patch", server::app_patch)
		.await
		.unwrap();
	module
		.declare_function("post", server::app_post)
		.await
		.unwrap();
	module
		.declare_function("put", server::app_put)
		.await
		.unwrap();
	module
		.declare_function("use", server::app_use)
		.await
		.unwrap();

	module
		.declare_function("listen", server::listen)
		.await
		.unwrap();

	module
		.declare_function("clearConfig", server::clear_config)
		.await
		.unwrap();

	module
		.call_function("logger.verbose", {
			let mut map = HashMap::new();
			map.insert(
				"data".to_owned(),
				juno::models::Value::String("Some data here".to_owned()),
			);
			map
		})
		.await
		.unwrap();

	loop {
		tokio::time::delay_for(Duration::from_millis(1000)).await;
	}
	*/
}
