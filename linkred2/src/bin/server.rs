#[macro_use]
extern crate log;

use log::LevelFilter;
use std::net::SocketAddr;
use structopt::StructOpt;
use linkred2::*;

//use linked2::*;
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Opt {
    #[structopt(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    default_value = "DEFAULT_LISTENING_ADDRESS",
    parse(try_from_str)
    )]
    addr: SocketAddr,
}

fn main(){
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    let mut opt = Opt::from_args();
    info!("Listening on {}", opt.addr);
    let sidecarServer = SidecarServer::new();
    sidecarServer.run(opt.addr);
}