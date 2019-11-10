extern crate env_logger;
/// Simple WebSocket client with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.
extern crate ws;
extern crate rust_docker;

use ws::{connect, CloseCode};
use rust_docker::api::containers::Containers;
use rust_docker::api::version::Version;
use rust_docker::client::DockerClient;

use std::process::exit;

fn main() {
    // Setup logging
    env_logger::init();
    let client = match DockerClient::new("unix:///var/run/docker.sock") {
      Ok(a) => a,
      Err(err) => {
          println!("{}", err);
          exit(1);
      }
    };
// Get version info for docker
let info = client.get_version_info();

// Get all containers(running/stopped)
let all_containers = client.list_all_containers(None).unwrap();

// Get only running containers
let running_cont = client.list_running_containers(None).unwrap();

println!("version_info: {:?}, all_containers: {:?}, running_count: {:?}", info, all_containers, running_cont);
    // Connect to the url and call the closure
    if let Err(error) = connect("ws://127.0.0.1:3012", |out| {
        // Queue a message to be sent when the WebSocket is open
        if out.send("Hello WebSocket").is_err() {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        // The handler needs to take ownership of out, so we use move
        move |msg| {
            // Handle messages received on this connection
            println!("Client got message '{}'. ", msg);

            // Close the connection
            out.close(CloseCode::Normal)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to: {:?}", error);
    }
}
