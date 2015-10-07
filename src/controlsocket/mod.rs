extern crate bufstream;
extern crate chrono;
use std::net::*;
use std::error::Error;
use std::io::{Read, Write, BufRead};
use std::io;
// use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown};
use self::bufstream::{BufStream, IntoInnerError};
use self::chrono::*;

// TODO store socket buffer size (1024?) as a constant out here somewhere
// socket buffer size should be at least 2048 re: tor dev list discussion

// TODO consistent error handling--don't just return err.to_string() or w/e


trait ControlSocket<'a> {
    fn send(&'a mut self, message: &str) -> Result<(), io::Error>;
    fn recv(&'a mut self) -> Result<usize, io::Error>;
    fn get_address(&'a mut self) -> String;
    fn get_port(&'a mut self) -> u16;
    // fn is_alive(&'a self) -> bool;
    fn is_localhost(&'a mut self) -> bool;
    fn connection_time(&'a mut self) -> Duration;
    // fn connect(&self); // TODO need this kind of stateful info? new == connect?
    fn close(self);
}

struct ControlPort {
    address: SocketAddr,
    buf_stream: BufStream<TcpStream>,
    buffer: Vec<u8>,
    time_connected: DateTime<UTC>
//    connected: bool
}

impl ControlPort {
    fn new(dest_addr: &str, dest_port: u16) -> ControlPort {
        // TODO handle this error, don't unwrap
        let stream = TcpStream::connect((dest_addr, dest_port)).unwrap();
        let address = stream.peer_addr().unwrap();
        let buf_stream = BufStream::new(stream);
        let vec_buffer = Vec::<u8>::with_capacity(2048);
        
        // return constructed instance
        ControlPort {
            // this should be ok if above ok
            address: address,
            buf_stream: buf_stream,
            buffer: vec_buffer,
            time_connected: UTC::now()
        }
    }
}

impl<'a> ControlSocket<'a> for ControlPort {
    fn get_address(&mut self) -> String {
        self.address.to_string()
    }
    fn get_port(&mut self) -> u16 {
        self.address.port()
    }
    fn send(&mut self, message: &str) -> Result<(), io::Error> {
        self.buf_stream.write_all(message.as_bytes())
    }
    fn recv(&mut self) -> Result<usize, io::Error> {
        self.buf_stream.read_until(b'\r', &mut self.buffer)
    }
    // fn is_alive(&mut self) -> bool {
    //     // TODO is this even necessary?
    // }
    fn is_localhost(&mut self) -> bool {
        // this just wraps the currently-unstable is_loopback() method for now
        match self.address.ip() {
            // enumerate over IpAddr enum since is_loopback() is not there yet
            IpAddr::V4(address) => address.is_loopback(),
            IpAddr::V6(address) => address.is_loopback()
        }
    }
    fn connection_time(&mut self) -> chrono::duration::Duration {
        chrono::UTC::now() - self.time_connected
    }
    fn close(self) {
        // TODO unwrap call here: need to handle two possible error types:
        // IntoInnerError<BufStream<TcpStream>> and io::Error
         // let tcp_stream = match self.buf_stream.into_inner() {
         //     Ok(stream) => stream,
         //     Err(err) => return err
         // };
         // match tcp_stream.shutdown(Shutdown::Both) {
         //     Ok(sb) => sb,
         //     Err(err) => return err
        // }
        let tcp_stream = self.buf_stream.into_inner()
            .unwrap()
            .shutdown(Shutdown::Both);
    }
}


#[cfg(test)]
mod tests {
    use super::{ControlPort, ControlSocket};

    // TODO refactor all of these
    
    // these two tests actually exercise the constructor and the three methods
    #[test]
    fn constructor_no_connection() {
        let test_ip = "127.0.0.1";
        let test_port = 9051;
        let mut test_control_port = ControlPort::new(test_ip, test_port);
        assert_eq!(test_control_port.get_address(), test_ip);
        assert_eq!(test_control_port.get_port(), test_port);
        assert!(test_control_port.is_localhost());
        // TODO figure out how to test valid state of buf_stream
        // assert!(!test_control_port.buf_stream);
    }

    // [should_panic(expected = "Could not connect with given addresses:")]
    #[test]
    #[should_panic]
    fn constructor_bad_addr() {
        // let test_ip = "9999999";
        // let test_port = 9051;
        // let test_control_port = ControlPort::new(test_ip, test_port);
    }
}
