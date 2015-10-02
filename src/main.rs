#![feature(ip)] // unstable is_loopback feature in ip
extern crate time;
mod controlsocket;

fn main() {
    let address = "127.0.0.1:9050".parse().unwrap();
    let server = TcpSocket::bind(&address).unwrap();
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server, CLIENT);
    let mut control_socket = controlsocket::ControlPort.new();
    event_loop.run(&mut control_socket);

}
