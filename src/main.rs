#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate futures;
extern crate gotham;
extern crate nickel;
extern crate tokio;

mod cli_parser;
mod http_config;
mod server;

use http_config::HttpConfig;

use std::collections::HashMap;
use std::sync::Mutex;

use nickel::ListeningServer;

use clap::{crate_name, crate_version};

lazy_static! {
	pub static ref HTTP_CONFIG: Mutex<Vec<HttpConfig>> = Mutex::new(Vec::new());
	pub static ref LISTENER: Mutex<Option<ListeningServer>> = Mutex::new(None);
}

#[tokio::main]
async fn main() {
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
}
