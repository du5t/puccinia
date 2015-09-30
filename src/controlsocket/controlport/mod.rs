#![feature(ip)] // unstable is_loopback feature in ip

use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

struct ControlPort {
    address: Ipv4Addr,
    port: u16,
    connect: bool,
    stream: TcpStream
}

impl ControlPort {
    fn new(dest_addr: &str, port: u16,
           connect: bool) -> ControlPort {
        let mut stream: TcpStream;
        if (connect == true) {
            stream = match TcpStream::connect((dest_addr, port)) {
                Ok(conn) => conn,
                Err(error) =>
                    panic!("Could not connect with given addresses: {}",
                           error)
            }
        }
        // convert ip addr string to actual type
        let address: Ipv4Addr = dest_addr.parse().unwrap();

        // return constructed instance
        ControlPort {
            address: address,
            port: port,
            connect: connect,
            stream: stream
        }
    }
    
    fn get_address(&self) -> String {
        self.address.to_string()
    }
    fn get_port(&self) -> i32 {
        self.port
    }
    fn is_localhost(&self) -> bool {
        self.address.is_loopback()
    }
}
