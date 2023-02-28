// Patrick Tracanelli
// simple test code to detect active udp socket (rustc udpcheck.rs)
use std::net::{UdpSocket, SocketAddr};

fn main() {
    let ip = "177.10.156.3"; // "capeta.freebsdbrasil.com.br";
    let socket_addr: SocketAddr = format!("{}:{}", ip, 53).parse().unwrap();

    // crate UDP socket
    let _sock = match UdpSocket::bind(socket_addr) {
        Ok(sock) => sock,
        Err(e) => panic!("Couldn't create socket: {}", e),
    };

    // bind
    if let Err(e) = UdpSocket::bind(socket_addr) {
        panic!("Could not bind socket: {}", e);
    }
}
