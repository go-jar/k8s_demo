use tokio::{
    time::Duration,
    net::UdpSocket,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let remote_addr = "10.112.0.130:30003".parse::<SocketAddr>().unwrap();
    let local_addr: SocketAddr = "0.0.0.0:0".parse::<SocketAddr>().unwrap();

    let socket = UdpSocket::bind(local_addr).await.unwrap();
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    socket.connect(&remote_addr).await.unwrap();

    loop {
        socket.send(b"hello world").await.unwrap();

        let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
        if let Ok(len) = socket.recv(&mut data).await {
            println!(
                "receive msg from server: {}", String::from_utf8_lossy(&data[..len])
            );
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
