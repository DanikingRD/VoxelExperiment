use std::net::{SocketAddr, UdpSocket};

pub fn bind_udp_socket(addr: SocketAddr) -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    Ok(socket)
}
