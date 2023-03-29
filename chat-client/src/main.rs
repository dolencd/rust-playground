use std::vec;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected");
    let mut stdin = io::stdin();
    let mut bytes_stdin: Vec<u8> = vec![0; 1024];
    let mut bytes_tcp: Vec<u8> = vec![0; 1024];

    loop {
        select! {
            stdin_bytes_read = stdin.read(&mut bytes_stdin) => {
                stream.write(&bytes_stdin[0..(stdin_bytes_read? - 1)]).await?;
            }
            tcp_bytes_received = stream.read(&mut bytes_tcp) => {
                let received = tcp_bytes_received?;

                if received == 0 {
                    println!("Socket closed. Exiting.");
                    break;
                }

                println!("Received: {}", std::str::from_utf8(&bytes_tcp[0..(received - 1)])?);
            }
        }
    }

    Ok(())
}
