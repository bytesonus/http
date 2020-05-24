use crate::{
	http_context::HttpContext,
	misc::{HttpConfig, HttpServerCommands, JunoCommands, MiddlewareType},
};
use futures::StreamExt;
use juno::{
	models::{Number, Value},
	Result,
};
use std::net::SocketAddr;
use thruster::{
	async_middleware, middleware_fn, App, Context, MiddlewareNext, MiddlewareResult, Request,
	Server, ThrusterServer,
};
use tokio::sync::{
	mpsc::{UnboundedReceiver, UnboundedSender},
	oneshot::channel,
};

pub async fn http_loop(
	mut http_receiver: UnboundedReceiver<HttpServerCommands>,
	juno_sender: UnboundedSender<JunoCommands>,
) {
	let mut http_config: Vec<HttpConfig> = vec![];
	while let Some(command) = http_receiver.next().await {
		match command {
			HttpServerCommands::AddConfig(config) => {
				http_config.push(config);
			}
			HttpServerCommands::ClearConfig => {
				http_config.clear();
			}
			HttpServerCommands::Listen(socket_addr) => {
				create_http_server(socket_addr, http_config.clone(), juno_sender.clone())
			}
		}
	}
}

fn create_http_server(
	socket_addr: SocketAddr,
	http_config: Vec<HttpConfig>,
	juno_sender: UnboundedSender<JunoCommands>,
) {
	let mut app =
		App::<Request, HttpContext, (Vec<HttpConfig>, UnboundedSender<JunoCommands>)>::create(
			generate_context,
			(http_config.clone(), juno_sender),
		);
	app.use_middleware("/", async_middleware!(HttpContext, [handle_middleware]));

	for config in http_config.iter() {
		let path = &config.path;
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
				app.patch(path, async_middleware!(HttpContext, [handle_data]));
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

	let server = Server::new(app);
	server.start(&format!("{}", socket_addr.ip()), socket_addr.port());
}

fn generate_context(
	request: Request,
	state: &(Vec<HttpConfig>, UnboundedSender<JunoCommands>),
	path: &str,
) -> HttpContext {
	let start = request.method().len() + 4;
	let matched_route = String::from(&path[start..]);

	HttpContext::new(request, matched_route, state.0.clone(), state.1.clone())
}

#[middleware_fn]
async fn handle_data(
	mut context: HttpContext,
	next: MiddlewareNext<HttpContext>,
) -> MiddlewareResult<HttpContext> {
	let fn_name = context.http_configs.iter().find(|config| {
		context.method() == config.config_type.to_capitalized_string()
			&& context.matched_route() == &config.path
	});
	if fn_name.is_none() {
		return next(context).await;
	}
	let fn_name = &fn_name.unwrap().func_name;

	let (sender, receiver) = channel::<Result<Value>>();

	let result = context.juno_sender.send(JunoCommands::CallFunction(
		fn_name.clone(),
		context.juno_representation().clone(),
		sender,
	));
	if result.is_err() {
		println!(
			"Error sending command across channels: {}",
			result.unwrap_err()
		);
		return next(context).await;
	}

	let response = receiver.await;
	if response.is_err() {
		println!(
			"Error receiving command across channels: {}",
			response.unwrap_err()
		);
		return next(context).await;
	}
	let response = response.unwrap();
	if response.is_err() {
		println!("Error calling function on juno: {}", response.unwrap_err());
		return next(context).await;
	}
	let response = response.unwrap();

	if response.as_object().is_none() {
		println!("Didn't get back object from function response");
		return next(context).await;
	}
	let response = response.as_object().unwrap();

	if response.get("next") == Some(&Value::Bool(true)) {
		if response.get("data").is_some() {
			context.data = response.get("data").unwrap().clone();
		}
		next(context).await
	} else {
		if let Some(Value::String(url)) = response.get("redirect") {
			context.redirect(url);
			return Ok(context);
		}

		if let Some(content_type) = response.get("contentType") {
			context.set_content_type(
				content_type
					.as_string()
					.unwrap_or(&String::from("text/html")),
			);
		}

		if let Some(Value::Object(headers)) = response.get("headers") {
			let _ = headers.iter().map(|item| {
				if item.1.is_string() {
					context.set(item.0, item.1.as_string().unwrap());
				}
			});
		}

		if let Some(Value::Number(Number::PosInt(num))) = response.get("status") {
			context.status(*num as u32);
		}

		if let Some(json) = response.get("json") {
			context.set_content_type("application/json");
			let json: serde_json::Value = json.clone().into();
			context.body(&json.to_string());
		} else {
			context.body(
				response
					.get("body")
					.unwrap_or(&Value::Null)
					.as_string()
					.unwrap_or(&String::from("")),
			);
		}

		Ok(context)
	}
}

#[middleware_fn]
async fn handle_middleware(
	mut context: HttpContext,
	next: MiddlewareNext<HttpContext>,
) -> MiddlewareResult<HttpContext> {
	let fn_name: Vec<&HttpConfig> = context
		.http_configs
		.iter()
		.filter(|config| {
			config.config_type == MiddlewareType::Use
				&& context.matched_route().starts_with(&config.path)
		})
		.collect();
	if fn_name.get(0).is_none() {
		return next(context).await;
	}
	let fn_name = &fn_name.get(0).unwrap().func_name;

	let (sender, receiver) = channel::<Result<Value>>();

	let result = context.juno_sender.send(JunoCommands::CallFunction(
		fn_name.clone(),
		context.juno_representation().clone(),
		sender,
	));
	if result.is_err() {
		println!(
			"Error sending command across channels: {}",
			result.unwrap_err()
		);
		return next(context).await;
	}

	let response = receiver.await;
	if response.is_err() {
		println!(
			"Error receiving command across channels: {}",
			response.unwrap_err()
		);
		return next(context).await;
	}
	let response = response.unwrap();
	if response.is_err() {
		println!("Error calling function on juno: {}", response.unwrap_err());
		return next(context).await;
	}
	let response = response.unwrap();

	if response.as_object().is_none() {
		println!("Didn't get back object from function response");
		return next(context).await;
	}
	let response = response.as_object().unwrap();

	if response.get("next") == Some(&Value::Bool(true)) {
		if response.get("data").is_some() {
			context.data = response.get("data").unwrap().clone();
		}
		next(context).await
	} else {
		if let Some(Value::String(url)) = response.get("redirect") {
			context.redirect(url);
			return Ok(context);
		}

		if let Some(content_type) = response.get("contentType") {
			context.set_content_type(
				content_type
					.as_string()
					.unwrap_or(&String::from("text/html")),
			);
		}

		if let Some(Value::Object(headers)) = response.get("headers") {
			let _ = headers.iter().map(|item| {
				if item.1.is_string() {
					context.set(item.0, item.1.as_string().unwrap());
				}
			});
		}

		if let Some(Value::Number(Number::PosInt(num))) = response.get("status") {
			context.status(*num as u32);
		}

		if let Some(json) = response.get("json") {
			context.set_content_type("application/json");
			let json: serde_json::Value = json.clone().into();
			context.body(&json.to_string());
		} else {
			context.body(
				response
					.get("body")
					.unwrap_or(&Value::Null)
					.as_string()
					.unwrap_or(&String::from("")),
			);
		}

		Ok(context)
	}
}
