extern crate futures;
extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            println!("addr {:?}", socket.peer_addr().unwrap());
            let buf = vec![0; 5];
            let conn = io::read_exact(socket, buf)
                .and_then(|(socket, buf)| io::write_all(socket, buf))
                .then(|_| Ok(()));
            tokio::spawn(conn);
            Ok(())
        })
        .map_err(|err| println!("err = {:?}", err));

    tokio::run(server)
}
