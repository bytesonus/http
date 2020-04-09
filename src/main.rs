#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate futures;
extern crate gotham;
extern crate nickel;
extern crate tokio;

mod http_config;

use http_config::HttpConfig;
use http_config::MiddlewareType;

use clap::{crate_authors, crate_name, crate_version, App, Arg};
use gotham::{models::Value, GothamModule};
use nickel::Nickel;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
	static ref HTTP_CONFIG: Mutex<Vec<HttpConfig>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
	let mut module = from_cli_args();
	module
		.initialize(crate_name!(), crate_version!(), HashMap::new())
		.await
		.unwrap();

	module
		.declare_function("clearConfig", clear_config)
		.await
		.unwrap();

	module.declare_function("use", app_use).await.unwrap();
}

fn app_use(args: HashMap<String, Value>) -> Value {
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

	HTTP_CONFIG.lock().unwrap().push(HttpConfig {
		config_type: MiddlewareType::USE,
		func_name: function.clone(),
		path: path.clone(),
	});

	Value::Null
}

fn listen(args: HashMap<String, Value>) -> Value {
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

fn clear_config(_args: HashMap<String, Value>) -> Value {
	Value::Null
}

#[allow(clippy::collapsible_if)]
pub fn from_cli_args() -> GothamModule {
	let args = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about("Http module")
		.arg(
			Arg::with_name("socket-location")
				.conflicts_with("port")
				.conflicts_with("host")
				.short("s")
				.long("socket-location")
				.takes_value(true)
				.value_name("FILE")
				.help("Sets the location of the socket to connect"),
		)
		.arg(
			Arg::with_name("port")
				.conflicts_with("socket-location")
				.short("p")
				.long("port")
				.takes_value(true)
				.value_name("PORT")
				.help("Sets the port for the socket to connect to"),
		)
		.arg(
			Arg::with_name("host")
				.conflicts_with("socket-location")
				.short("h")
				.long("host")
				.takes_value(true)
				.value_name("HOST-IP")
				.help("Sets the host address for the socket to connect"),
		)
		.arg(
			Arg::with_name("V")
				.short("V")
				.multiple(true)
				.help("Sets the level of verbosity (max 3)"),
		)
		.arg(
			Arg::with_name("version")
				.short("v")
				.long("version")
				.help("Prints version information"),
		)
		.get_matches();

	if args.is_present("version") {
		println!("{}", crate_version!());
		panic!();
	}

	let mut default_socket_location = std::env::current_dir().unwrap();
	default_socket_location.push(args.value_of("socket-location").unwrap_or("../gotham.sock"));
	let default_socket_location = default_socket_location.as_os_str().to_str().unwrap();

	if cfg!(target_family = "windows") {
		if args.value_of("socket-location").is_some() {
			panic!("Listening on unix sockets are not supported on windows");
		} else {
			GothamModule::from_inet_socket(
				args.value_of("host").unwrap_or("127.0.0.1"),
				args.value_of("port")
					.unwrap_or("2203")
					.parse::<u16>()
					.unwrap(),
			)
		}
	} else {
		if args.value_of("port").is_some() {
			GothamModule::from_inet_socket(
				args.value_of("host").unwrap_or("127.0.0.1"),
				args.value_of("port")
					.unwrap_or("2203")
					.parse::<u16>()
					.unwrap(),
			)
		} else {
			GothamModule::from_unix_socket(
				args.value_of("socket-location")
					.unwrap_or(default_socket_location),
			)
		}
	}
}
