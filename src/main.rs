#![feature(ip)] // unstable is_loopback feature in ip
mod controlsocket;

fn main() {
    let address = "127.0.0.1:9050".parse().unwrap();
    // let server = TcpSocket::bind(&address).unwrap();

}
