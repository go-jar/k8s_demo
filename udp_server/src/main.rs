use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:3333").unwrap();

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 11];

    loop {
        if let Ok((_amt, src)) = socket.recv_from(&mut buf) {
            println!(
                "receive msg from client: {:?}",
                String::from_utf8_lossy(&buf)
            );
            socket.send_to(&buf, &src).unwrap();
        }
    }
}
