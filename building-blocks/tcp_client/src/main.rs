use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:9123") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9123");

            const BUFFER_SIZE: usize = 10;
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let mut raw_resp: Vec<u8> = Vec::new();
            let mut total = 0;
            loop {
                let len = match stream.read(&mut buffer) {
                    Ok(len) => len,
                    Err(_) => 0,
                };

                for i in 0..len {
                    raw_resp.push(buffer[i]);
                }
                total+=len;

                if len < BUFFER_SIZE {
                    break;
                }
            }
            println!("read: {}totol:{}", std::str::from_utf8(raw_resp.as_slice()).unwrap(), total);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}