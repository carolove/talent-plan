extern crate bytes;
extern crate tokio;
#[macro_use]
extern crate futures;
use bytes::{Buf, Bytes};
use futures::{Async, Future, Poll};
use std::io::{self, Cursor};
use tokio::io::AsyncWrite;
use tokio::net::{tcp::ConnectFuture, TcpStream};
// HelloWorld 有两个状态, 即等待连接的状态和已经连接的状态
enum HelloWorld {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}
impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;
    fn poll(&mut self) -> Poll<(), io::Error> {
        use self::HelloWorld::*;
        loop {
            let socket = match *self {
                Connecting(ref mut f) => {
                    try_ready!(f.poll())
                }
                Connected(ref mut socket, ref mut data) => {
                    // 只要缓冲区还有可用的数据，就一直将其写入到套接字中
                    while data.has_remaining() {
                        try_ready!(socket.write_buf(data));
                    }
                    return Ok(Async::Ready(()));
                }
            };
            let data = Cursor::new(Bytes::from_static(b"hello world"));
            *self = Connected(socket, data);
        }
    }
}
fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let hello_world = HelloWorld::Connecting(connect_future);
    // 运行之
    tokio::run(hello_world.map_err(|e| println!("{0}", e)))}
