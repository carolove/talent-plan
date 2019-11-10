extern crate env_logger;
extern crate log;
use log::{error, info};

fn main() {
    env_logger::init();

    println!("hello world!");
    info!("hello world info!");
    error!("the world is empty!");
}
