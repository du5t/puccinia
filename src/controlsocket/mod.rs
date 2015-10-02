use std::net::{TcpStream, SocketAddr};
use bufstream::BufStream;
use time::duration
// use std::thread;

// TODO store socket buffer size (1024?) as a constant out here somewhere

trait ControlSocket {
    fn send(message: &str, raw: bool);
    fn recv(&self) -> ControlMessage;
    fn is_alive(&self) -> bool;
    fn is_localhost(&self) -> bool;
    fn connection_time(&self) -> f64;
    fn connect(&self);
    fn close(&self);
}

struct ControlPort {
    address: SocketAddr,
    buf_stream: BufStream,
    buffer: Vec<T>,
    connect_time: time::Tm
//    connected: bool
}

impl ControlPort {
    fn new(dest_addr: &str, dest_port: u16) -> ControlPort {
        let stream = TcpStream::connect(dest_addr, dest_port).unwrap();
        let bufstream = BufStream::new(stream);

        // return constructed instance
        ControlPort {
            address: stream.peer_addr().unwrap(),
            buf_stream: buf_stream,
            buffer: Vec::with_capacity(1024),
            connect_time: time::now_utc()
        }
    }
}

impl ControlPort for ControlSocket {
    fn get_address(&self) -> String {
        self.address.to_string()
    }
    fn get_port(&self) -> u16 {
        self.address.port()
    }
    fn send(&self, message: &str, raw: bool) -> Result<usize> {
        self.socket.write_all(message).unwrap();
    }
    pub fn recv(&self) -> Result<usize> {
        self.read_until("\r", self.buffer)
    }
    pub fn is_alive(&self) -> bool {
        // TODO is this even necessary?
    }
    pub fn is_localhost(&self) -> bool {
        let my_ip = self.address.ip();
        match my_ip {
            "127.0.0.1" => true,
            "::1" => true,
            _ => false
        }
    }
    pub fn connection_time(&self) -> Duration {
        self.connect_time.sub(time::now_utc())
    }
    pub fn close(&self) {
        let tcp_stream = self.buf_stream.into_inner();
        tcp_stream.shutdown()
    };
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
        let test_control_port = ControlPort::new(test_ip, test_port, false);
        assert_eq!(test_control_port.get_address(), test_ip);
        assert_eq!(test_control_port.get_port(), test_port);
        assert!(test_control_port.is_localhost());
        assert!(!test_control_port.stream.is_some());
    }

    #[test]
    fn constructor_connection() {
        let test_ip = "127.0.0.1";
        let test_port = 9051;
        let test_control_port = ControlPort::new(test_ip, test_port, true);
        assert!(test_control_port.stream.is_some());
        // assert_eq!(resolved_stream.peer_addr(), test_ip);
    }

    #[test]
    #[should_panic(expected = "Could not connect with given addresses:")]
    fn constructor_bad_addr() {
        let test_ip = "9999999";
        let test_port = 9051;
        let test_control_port = ControlPort::new(test_ip, test_port, true);
    }
}
