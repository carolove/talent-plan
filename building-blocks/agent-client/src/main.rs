extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    // Parse the address of whatever server we're talking to
    let addr = "127.0.0.1:12345".parse().unwrap();

    // Following snippets come here...
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            // We're going to read the first 32 bytes the server sends us
            // and then just echo them back:
            let mut buf = vec![0; 32];
            // First, we need to read the server's message
            tokio::io::read_exact(stream, buf)
        })
        .and_then(|(stream, buf)| {
            // Then, we use write_all to write the entire buffer back:
            tokio::io::write_all(stream, buf)
        })
        .inspect(|(_stream, buf)| {
            println!("echoed back {} bytes: {:x?}", buf.len(), buf);
        })
        .map_err(|err| {
            // All tasks must have an 'Error' type of '()'. This forces error
            // handing and helps avoid silencing failures.
            println!("connection error = {:?}",err);
        });

    println!("About to create the stream and write to it...");
    tokio::run(client);
    println!("Stream has been created and written to.");
}