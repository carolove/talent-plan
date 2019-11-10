extern crate futures;
extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::{Future, Async, Poll};
use std::fmt;

struct Display<T>(T);
impl<T> Future for Display<T>
    where
        T: Future,
        T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = match self.0.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(error) => return Err(error),
        };
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}

struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        return Ok(Async::Ready("hello world".to_string()));
    }
}

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let amount = io::copy(reader, writer);

            // futures 其实就相当于一个异步代码块，spawn是这部分代码能够异步执行的关键否则为同步代码
            let msg = amount.then(|result| {
                match result {
                    Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                    Err(e) => println!("error : {}", e),
                }

                Ok(())
            });
            tokio::spawn(msg);

            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {}", err);
        });
    println!("display hello world future");
    let future = Display(HelloWorld);
    tokio::run(future);
    println!("server running at localhost:6142");
    tokio::run(server);
}
