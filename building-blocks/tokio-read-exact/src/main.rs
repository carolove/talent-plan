extern crate tokio;
use tokio::net::tcp::TcpStream;
use tokio::prelude::*;

fn main() {

    let addr = "127.0.0.1:8080".parse().unwrap();
    let write_read_8_fut = TcpStream::connect(&addr)
        .and_then(|stream| {
            let buf = "helloworld".as_bytes();
            // Then, we use write_all to write the entire buffer back:
            tokio::io::write_all(stream, buf)
        })
        .and_then(|(stream,_)| {
            // We need to create a buffer for read_exact to write into.
            // A Vec<u8> is a good starting point.
            // read_exact will read buffer.len() bytes, so we need
            // to make sure the Vec isn't empty!
            let mut buf = vec![0; 5];

            // read_exact returns a Future that resolves when
            // buffer.len() bytes have been read from stream.
            tokio::io::read_exact(stream, buf)
        })
        .then(|_|{Ok(())});

    tokio::run(write_read_8_fut)
}
