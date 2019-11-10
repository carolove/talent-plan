use docker_sock_tcp_raw::api::containers::Containers;
use docker_sock_tcp_raw::api::version::Version;
use docker_sock_tcp_raw::client::DockerClient;

use std::process::exit;
use std::io::{Write, Read};
use std::net::TcpListener;
use std::thread;
use docker_sock_tcp_raw::api::rawreq::Rawreq;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            match stream {
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
                    let client = match DockerClient::new("unix:///var/run/docker.sock") {
                        Ok(a) => a,
                        Err(err) => {
                            println!("{}", err);
                            exit(1);
                        }
                    };
                    let info = client.get_raw_req( std::str::from_utf8(raw_resp.as_slice()).unwrap()).unwrap();
                    stream.write_all(info.as_bytes());
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
        });
    }
}
