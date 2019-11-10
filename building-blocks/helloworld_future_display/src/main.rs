#[macro_use]
extern crate futures;
use futures::{Future, Poll, Async};
use std::fmt;
struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        return Ok(Async::Ready("hello world".to_string()));
    }
}

// A tuple struct, instead of a field struct.
struct Display<T>(T);
impl<T> Future for Display<T>
    where
        T: Future,
        T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = try_ready!(self.0.poll());
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}

fn main() {
    let future = Display(HelloWorld);
    tokio::run(future);
}