extern crate bufstream;
extern crate chrono;
extern crate unix_socket;
use std::net::*;
use std::error::Error;
use std::io::{Write, BufRead}; // Read also?
use std::io;
// use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown};
use self::bufstream::{BufStream}; // IntoInnerError also once we handle it well
use self::chrono::*;
use self::unix_socket::UnixStream;

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
    fn close(self) -> Result<(), io::Error>;
}

struct DomainSocket {
    socket_path: String,
    buf_stream: BufStream<UnixStream>,
    buffer: Vec<u8>,
    time_connected: DateTime<UTC>
}

struct ControlPort {
    address: SocketAddr,
    buf_stream: BufStream<TcpStream>,
    buffer: Vec<u8>,
    time_connected: DateTime<UTC>
}

impl DomainSocket {
    fn new(dest_path: &str) -> io::Result<DomainSocket> {
        let socket = try!(UnixStream::connect(dest_path));
        let buf_stream = BufStream::new(socket);
        let vec_buffer = Vec::<u8>::with_capacity(2048);
        Ok(DomainSocket {
            socket_path: dest_path.to_string(),
            buf_stream: buf_stream,
            buffer: vec_buffer,
            time_connected: UTC::now()
        })
    }
}

impl<'a> ControlSocket<'a> for DomainSocket {
    fn get_address(&mut self) -> String {
        self.socket_path.to_string()
    }
    fn get_port(&mut self) -> u16 {
        // TODO express better that sockets have no port or otherwise refactor
        return 0;
    }
    fn send(&'a mut self, message: &str) -> Result<(), io::Error> {
        self.buf_stream.write_all(message.as_bytes())
    }
    fn recv(&'a mut self) -> Result<usize, io::Error> {
        self.buf_stream.read_until(b'\r', &mut self.buffer)
    }
    // fn is_alive(&'a self) -> bool;
    fn is_localhost(&mut self) -> bool {
        // TODO if there is some crazy remote socket stuff going on, check for
        // it
        true
    }
    fn connection_time(&'a mut self) -> Duration {
        chrono::UTC::now() - self.time_connected
    }
    // fn connect(&self); // TODO need this kind of stateful info? new == connect?
    fn close(self) -> Result<(), io::Error>{
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
        self.buf_stream.into_inner()
            .unwrap()
            .shutdown(Shutdown::Both)
    }
}

impl ControlPort {
    fn new(dest_addr: &str, dest_port: u16) -> io::Result<ControlPort> {
        let stream = try!(TcpStream::connect((dest_addr, dest_port)));
        // this should be ok if above ok
        let address = stream.peer_addr().unwrap();
        let buf_stream = BufStream::new(stream);
        let vec_buffer = Vec::<u8>::with_capacity(2048);
        
        // return constructed instance
        Ok(ControlPort {
            address: address,
            buf_stream: buf_stream,
            buffer: vec_buffer,
            time_connected: UTC::now()
        })
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
    fn connection_time(&mut self) -> Duration {
        chrono::UTC::now() - self.time_connected
    }
    fn close(self) -> Result<(), io::Error>{
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
        self.buf_stream.into_inner()
            .unwrap()
            .shutdown(Shutdown::Both)
    }
}


#[cfg(test)]
mod tests {
    use super::{ControlPort, ControlSocket, DomainSocket};

    #[test]
    fn controlport_constructor() {
        let test_ip = "127.0.0.1";
        let test_port = 9051;
        let ip_string = test_ip.to_string() + ":" + &(test_port).to_string();
        println!("{}", ip_string);
        let mut test_control_port = ControlPort::new(test_ip, test_port)
            .unwrap();
        assert_eq!(test_control_port.get_address(), ip_string);
        assert_eq!(test_control_port.get_port(), test_port);
        assert!(test_control_port.is_localhost());
        // TODO figure out how to test valid state of buf_stream
        // assert!(!test_control_port.buf_stream);
    }

    // [should_panic(expected = "Could not connect with given addresses:")]
    #[test]
    #[should_panic]
    fn controlport_constructor_bad_addr() {
        let test_ip = "9999999";
        let test_port = 9051;
        let test_control_port = ControlPort::new(test_ip, test_port)
            .unwrap();
    }

    #[test]
    fn controlport_destructor() {
        let test_ip = "127.0.0.1";
        let test_port = 9051;
        let ip_string = test_ip.to_string() + ":" + &(test_port).to_string();
        println!("{}", ip_string);
        let mut test_control_port = ControlPort::new(test_ip, test_port)
            .unwrap();
        test_control_port.close().ok().expect("Should have closed properly.")
    }

    #[test]
    fn domainsocket_constructor() {
        let test_sockfile = "./torsocket";
        let mut test_domain_socket = DomainSocket::new(test_sockfile)
            .unwrap();
        assert_eq!(test_domain_socket.get_address(), test_sockfile);
        assert_eq!(test_domain_socket.get_port(), 0);
        assert!(test_domain_socket.is_localhost());
        // TODO figure out how to test valid state of buf_stream
        // assert!(!test_domain_socket.buf_stream);
    }

    // [should_panic(expected = "Could not connect with given addresses:")]
    #[test]
    #[should_panic]
    fn domainsocket_constructor_bad_path() {
        let test_path = "/foo/bar";
        let test_domain_socket = DomainSocket::new(test_path)
            .unwrap();
    }

    #[test]
    fn domainsocket_destructor() {
        let test_sockfile = "./torsocket";
        let mut test_domain_socket = DomainSocket::new(test_sockfile)
            .unwrap();        
        test_domain_socket.close().ok().expect("Should have closed properly.")
    }
}
