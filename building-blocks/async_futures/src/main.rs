extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

use tokio::io::AsyncWrite;
use futures::{Async, Future, Poll};
use tokio::net::{tcp::ConnectFuture, TcpStream};
use bytes::{Bytes, Buf};
use std::io::{self, Cursor};

struct GetPeerAddr {
    connect : ConnectFuture,
}

impl Future for GetPeerAddr {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.connect.poll() {
            Ok(Async::Ready(socket)) => {
                println!("peer address = {}",
                         socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                println!("failed to connect: {}", e);
                Ok(Async::Ready(()))
            }
        }
    }
}

fn main() {
    let addr = "192.168.0.1:1234".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let get_peer_addr = GetPeerAddr {
        connect: connect_future,
    };
    tokio::run(get_peer_addr);
}