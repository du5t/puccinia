use std::net::{Ipv4Addr, TcpStream};

struct ControlPort {
    address: Ipv4Addr,
    port: u16,
    connect: bool,
    stream: Option<TcpStream> // optional because it does not register
}

impl ControlPort {
    fn new(dest_addr: &str, port: u16, connect: bool) -> ControlPort {
        let stream: Option<TcpStream>;

        if connect == true {
            stream = match TcpStream::connect((dest_addr, port)) {
                Ok(conn) => Some(conn),
                Err(error) =>
                    panic!("Could not connect with given addresses: {}", error)
            };
        } else {
            stream = None;
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
    fn get_port(&self) -> u16 {
        self.port
    }
    fn is_localhost(&self) -> bool {
        self.address.is_loopback()
    }
}

#[cfg(test)]
mod tests {
    use super::ControlPort;
    
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
