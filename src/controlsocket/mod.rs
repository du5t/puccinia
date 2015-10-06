extern crate bufstream;
extern crate chrono;
use std::net::*;
use std::io::{Read, Write, BufRead, Error, IntoInnerError};
// use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown};
use self::bufstream::BufStream;
use self::chrono::*;

// TODO store socket buffer size (1024?) as a constant out here somewhere
// socket buffer size should be at least 2048 re: tor dev list discussion

// TODO consistent error handling--don't just return err.to_string() or w/e


trait ControlSocket<'a> {
    fn send(&'a self, message: &str) -> Result<usize, Error>;
    fn recv(&'a self) -> Result<usize, Error>;
    fn get_address(&'a self) -> String;
    fn get_port(&'a self) -> u16;
    fn is_alive(&'a self) -> bool;
    fn is_localhost(&'a self) -> bool;
    fn connection_time(&'a self) -> Duration;
    // fn connect(&self); // TODO need this kind of stateful info? new == connect?
    fn close(&'a self) -> Result<(), IntoInnerError<BufStream<TcpStream>>>;
}

struct ControlPort<'a> {
    address: SocketAddr,
    buf_stream: BufStream<TcpStream>,
    buffer: &'a Vec<u8>,
    time_connected: DateTime<UTC>
//    connected: bool
}

impl<'a> ControlPort<'a> {
    fn new(dest_addr: &str, dest_port: u16) -> ControlPort {
        let stream = try!(TcpStream::connect((dest_addr, dest_port)));
        let buf_stream = BufStream::new(stream);
        let mut vec_buffer = Vec::<u8>::with_capacity(2048);
        
        // return constructed instance
        ControlPort {
            address: stream.peer_addr().unwrap(), // this should be ok if above ok
            buf_stream: buf_stream,
            buffer: &mut vec_buffer,
            time_connected: UTC::now()
        }
    }
}

impl<'a> ControlSocket<'a> for ControlPort<'a> {
    fn get_address(&self) -> String {
        self.address.to_string()
    }
    fn get_port(&self) -> u16 {
        self.address.port()
    }
    fn send(&self, message: &str) -> Result<usize, Error> {
        try!(self.buf_stream.write_all(message.as_bytes()));
    }
        try!(self.buf_stream.read_until(b'\r', &self.buffer))
    pub fn recv(&self) -> Result<usize, Error> {
    }
    pub fn is_alive(&self) -> bool {
        // TODO is this even necessary?
    }
    pub fn is_localhost(&self) -> bool {
        // this just wraps the currently-unstable is_loopback() method for now
        match self.address.ip() {
            // enumerate over IpAddr enum since is_loopback() is not there yet
            IpAddr::V4(address) => address.is_loopback(),
            IpAddr::V6(address) => address.is_loopback()
        }
    }
    pub fn connection_time(&self) -> chrono::duration::Duration {
        chrono::UTC::now() - self.time_connected
    }
    pub fn close(&self) -> Result<(), IntoInnerError<BufStream<TcpStream>>> {
        let tcp_stream = try!(self.buf_stream.into_inner());
        tcp_stream.shutdown(Shutdown::Both)
    }
}


#[cfg(test)]
mod tests {
    use super::ControlPort;

    // TODO refactor all of these
    
    // these two tests actually exercise the constructor and the three methods
    #[test]
    fn constructor_no_connection() {
        let test_ip = "127.0.0.1";
        let test_port = 9051;
        let test_control_port = ControlPort::new(test_ip, test_port);
        assert_eq!(test_control_port.get_address(), test_ip);
        assert_eq!(test_control_port.get_port(), test_port);
        assert!(test_control_port.is_localhost());
        assert!(!test_control_port.buf_stream.is_some());
    }

    // [should_panic(expected = "Could not connect with given addresses:")]
    #[test]
    #[should_panic]
    fn constructor_bad_addr() {
        let test_ip = "9999999";
        let test_port = 9051;
        let test_control_port = ControlPort::new(test_ip, test_port);
    }
}
