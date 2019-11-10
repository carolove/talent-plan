use crate::{Result,Request};
use serde_json::Deserializer;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{BufReader, BufWriter, Write};

/// The server of a key value store.
pub struct SidecarServer {
}

impl SidecarServer {
    /// new sidecar server
    pub fn new() -> Self { SidecarServer{} }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let peer_addr = tcp.peer_addr()?;
        let reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        for req in req_reader {
            let req = req?;
            debug!("Receive request from {}: {:?}", peer_addr, req);
        }

        Ok(())
    }
}