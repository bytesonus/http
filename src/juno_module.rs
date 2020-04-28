use crate::misc::{HttpConfig, HttpServerCommands, JunoCommands, MiddlewareType};
use clap::{crate_authors, crate_name, crate_version, App, Arg};
use futures::StreamExt;
use juno::{
	models::{Number, Value},
	JunoModule,
};
use std::{
	collections::HashMap,
	net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{
	runtime::Runtime,
	sync::{
		mpsc::{UnboundedReceiver, UnboundedSender},
		Mutex,
	},
};

lazy_static! {
	static ref HTTP_SENDER: Mutex<Option<UnboundedSender<HttpServerCommands>>> = Mutex::new(None);
}

#[allow(clippy::collapsible_if)]
fn from_cli_args() -> JunoModule {
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
	default_socket_location.push(args.value_of("socket-location").unwrap_or("../juno.sock"));
	let default_socket_location = default_socket_location.as_os_str().to_str().unwrap();

	if cfg!(target_family = "windows") {
		if args.value_of("socket-location").is_some() {
			panic!("Listening on unix sockets are not supported on windows");
		} else {
			JunoModule::from_inet_socket(
				args.value_of("host").unwrap_or("127.0.0.1"),
				args.value_of("port")
					.unwrap_or("2203")
					.parse::<u16>()
					.unwrap(),
			)
		}
	} else {
		if args.value_of("port").is_some() {
			JunoModule::from_inet_socket(
				args.value_of("host").unwrap_or("127.0.0.1"),
				args.value_of("port")
					.unwrap_or("2203")
					.parse::<u16>()
					.unwrap(),
			)
		} else {
			JunoModule::from_unix_socket(
				args.value_of("socket-location")
					.unwrap_or(default_socket_location),
			)
		}
	}
}

pub async fn juno_loop(
	mut juno_receiver: UnboundedReceiver<JunoCommands>,
	http_sender: UnboundedSender<HttpServerCommands>,
) {
	HTTP_SENDER.lock().await.replace(http_sender);

	let mut module = from_cli_args();
	module
		.initialize(crate_name!(), crate_version!(), HashMap::new())
		.await
		.unwrap();

	module
		.declare_function("delete", |args| {
			add_app_command(MiddlewareType::Delete, args)
		})
		.await
		.unwrap();
	module
		.declare_function("get", |args| add_app_command(MiddlewareType::Get, args))
		.await
		.unwrap();
	module
		.declare_function("options", |args| {
			add_app_command(MiddlewareType::Options, args)
		})
		.await
		.unwrap();
	module
		.declare_function("patch", |args| add_app_command(MiddlewareType::Patch, args))
		.await
		.unwrap();
	module
		.declare_function("post", |args| add_app_command(MiddlewareType::Post, args))
		.await
		.unwrap();
	module
		.declare_function("put", |args| add_app_command(MiddlewareType::Put, args))
		.await
		.unwrap();
	module
		.declare_function("use", |args| add_app_command(MiddlewareType::Use, args))
		.await
		.unwrap();

	module
		.declare_function("listen", listen_command)
		.await
		.unwrap();

	module
		.declare_function("clearConfig", clear_app_config)
		.await
		.unwrap();
	while let Some(data) = juno_receiver.next().await {
		match data {
			JunoCommands::CallFunction(function_name, args, response_sender) => {
				let result = response_sender.send(module.call_function(&function_name, args).await);
				if result.is_err() {
					println!(
						"Error sending function_call response: {:#?}",
						result.unwrap_err()
					);
				}
			}
			JunoCommands::TriggerHook(hook_name) => {
				let result = module.trigger_hook(&hook_name).await;
				if result.is_err() {
					println!("Error triggering hook: {}", result.unwrap_err());
				}
			}
		}
	}
}

fn add_app_command(middleware_type: MiddlewareType, args: HashMap<String, Value>) -> Value {
	Runtime::new().unwrap().block_on(async {
		let sender = HTTP_SENDER.lock().await;
		let sender = sender.as_ref().unwrap();

		let string = Value::String("/".to_string());
		let path = String::from("/");
		let path = args
			.get("path")
			.unwrap_or(&string)
			.as_string()
			.unwrap_or(&path);

		let func_name = args.get("function").unwrap_or(&Value::Null).as_string();
		if func_name.is_none() {
			return;
		}

		let result = sender.send(HttpServerCommands::AddConfig(HttpConfig {
			config_type: middleware_type,
			func_name: func_name.unwrap().clone(),
			path: path.clone(),
		}));
		if result.is_err() {
			println!("Error sending add-config command: {}", result.unwrap_err());
		}
	});
	Value::Null
}

fn listen_command(args: HashMap<String, Value>) -> Value {
	Runtime::new().unwrap().block_on(async {
		let sender = HTTP_SENDER.lock().await;
		let sender = sender.as_ref().unwrap();

		let string = Value::String("127.0.0.1".to_string());
		let socket = String::from("127.0.0.1");
		let socket = args
			.get("socket")
			.unwrap_or(&string)
			.as_string()
			.unwrap_or(&socket);

		let port = args
			.get("port")
			.unwrap_or(&Value::Number(Number::PosInt(3000)))
			.as_number()
			.unwrap_or(&Number::PosInt(3000))
			.as_u64()
			.unwrap_or(3000) as u16;

		let result = sender.send(HttpServerCommands::Listen(SocketAddr::new(
			socket.parse().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
			port,
		)));
		if result.is_err() {
			println!("Error sending listen command: {}", result.unwrap_err());
		}
	});
	Value::Null
}

fn clear_app_config(_: HashMap<String, Value>) -> Value {
	let result = Runtime::new()
		.unwrap()
		.block_on(HTTP_SENDER.lock())
		.as_ref()
		.unwrap()
		.send(HttpServerCommands::ClearConfig);

	if result.is_err() {
		println!(
			"Error sending clear-config command: {}",
			result.unwrap_err()
		);
	}
	Value::Null
}
