use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    select,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let (source_sender, source_receiver) = broadcast::channel::<Vec<u8>>(42);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on {}", address);
    loop {
        let (mut socket, addr) = listener.accept().await?;

        let sender = source_sender.clone();
        let mut receiver = source_receiver.resubscribe();
        tokio::spawn(async move {
            println!("Got a connection from: {}", addr);

            let mut data: Vec<u8> = vec![0; 1024];

            loop {
                select! {
                    bytes_read_future = socket.read(&mut data) => {
                        let bytes_read = bytes_read_future?;

                        if bytes_read == 0 {
                            break;
                        }

                        let read_data = data[0..(bytes_read)].to_owned();

                        println!(
                            "Got message: {} with length: {}",
                            std::str::from_utf8(&read_data)?,
                            bytes_read
                        );

                        sender.send(read_data)?;
                    }
                    to_send = receiver.recv() => {
                        socket.write(&to_send?).await?;
                    }
                }
            }

            println!("Client disconnected: {}", addr);
            anyhow::Ok(())
        });
    }
}
