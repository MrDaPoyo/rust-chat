use tokio::net::UdpSocket;
use tokio::io::{self, AsyncBufReadExt};

const BROADCAST_ADDR: &str = "255.255.255.255:10080";
const LISTEN_ADDR: &str = "0.0.0.0:10080";

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind(LISTEN_ADDR).await?;
    socket.set_broadcast(true)?;

    println!("Listening on: {}", LISTEN_ADDR);

    let send_socket = UdpSocket::bind("0.0.0.0:0").await?;
    send_socket.set_broadcast(true)?;
    let recv_socket = socket;

    tokio::spawn(async move {
        let mut buffer = vec![0; 1024];
        loop {
            match recv_socket.recv_from(&mut buffer).await {
                Ok((len, addr)) => {
                    let msg = String::from_utf8_lossy(&buffer[..len]);
                    println!("[{}]: {}", addr, msg);
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }    
        }
    });

    let stdin = io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    println!("Type your messages below:");
    while let Some(line) = lines.next_line().await? {
        send_socket.send_to(line.as_bytes(), BROADCAST_ADDR).await?;
    }

    Ok(())
}