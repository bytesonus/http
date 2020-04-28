#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate clap;
extern crate futures;
extern crate juno;
extern crate serde_json;
extern crate thruster;
extern crate tokio;

mod http_context;
mod http_server;
mod juno_module;
mod misc;

use misc::{HttpConfig, HttpServerCommands, JunoCommands};

use std::time::Duration;
use tokio::{sync::mpsc, time};

#[tokio::main]
async fn main() {
	let (juno_sender, juno_receiver) = mpsc::unbounded_channel::<JunoCommands>();
	let (http_sender, http_receiver) = mpsc::unbounded_channel::<HttpServerCommands>();

	tokio::spawn(juno_module::juno_loop(juno_receiver, http_sender));
	tokio::spawn(http_server::http_loop(http_receiver, juno_sender));

	loop {
		time::delay_for(Duration::from_millis(1000)).await;
	}
}
