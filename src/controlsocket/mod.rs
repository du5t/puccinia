extern crate bufstream;
extern crate chrono;
// use std::net::*;
use std::net::{TcpStream, SocketAddr, Ipv4Addr, Ipv6Addr};
use self::bufstream::BufStream;
use self::chrono::*;

// TODO store socket buffer size (1024?) as a constant out here somewhere
// socket buffer size should be at least 2048 re: tor dev list discussion

// TODO consistent error handling--don't just return err.to_string() or w/e
use std::fmt::Display;
trait Error: Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error> { None }
}

trait ControlSocket {
    fn send(&self, message: &str) -> Result<usize, Display>;
    fn recv(&self) -> Result<usize, Display>;
    fn get_address(&self) -> String;
    fn get_port(&self) -> u16;
    fn is_alive(&self) -> bool;
    fn is_localhost(&self) -> bool;
    fn connection_time(&self) -> Duration;
    // fn connect(&self); // TODO need this kind of stateful info? new == connect?
    fn close(&self) -> Result<(), Display>;
}

struct ControlPort {
    address: SocketAddr,
    buf_stream: BufStream<TcpStream>,
    buffer: Vec<()>,
    connect_time: chrono::duration::Duration
//    connected: bool
}

impl ControlPort {
    fn new(dest_addr: &str, dest_port: u16) -> ControlPort {
        let stream = try!(TcpStream::connect((dest_addr, dest_port)));
        let buf_stream = BufStream::new(stream);

        // return constructed instance
        ControlPort {
            address: stream.peer_addr().unwrap(), // this should be ok if above ok
            buf_stream: buf_stream,
            buffer: Vec::with_capacity(2048),
            connect_time: chrono::duration::Duration::zero()
        }
    }
}

impl ControlSocket for ControlPort {
    fn get_address(&self) -> String {
        self.address.to_string()
    }
    fn get_port(&self) -> u16 {
        self.address.port()
    }
    fn send(&self, message: &str) -> Result<usize, Display> {
        try!(self.buf_stream.write_all(message, self.buffer));
    }
    pub fn recv(&self) -> Result<usize, Display> {
        try!(self.buf_stream.read_until("\r", self.buffer))
    }
    pub fn is_alive(&self) -> bool {
        // TODO is this even necessary?
    }
    pub fn is_localhost(&self) -> bool {
        // this just wraps the currently-unstable is_loopback() method for now
        match self.address.ip() {
            // enumerate over IpAddr enum since is_loopback() is not there yet
            Ipv4Addr(address) => address.is_loopback(),
            Ipv6Addr(address) => address.is_loopback()
        }
    }
    pub fn connection_time(&self) -> chrono::duration::Duration {
        self.connect_time.sub(chrono::datetime::DateTime::now_utc())
    }
    pub fn close(&self) -> Result<(), Display> {
        let tcp_stream = self.buf_stream.into_inner();
        tcp_stream.shutdown()
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
