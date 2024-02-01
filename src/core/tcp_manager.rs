use crate::core::store::{Status, Store};
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct TcpManager;

impl TcpManager {
    pub async fn start() -> io::Result<()> {
        let config = Store::new().get_config();
        let port = config.port;
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut socket, _) = listener.accept().await?;
            let mut buffer = [0; 1];
            socket.read(&mut buffer).await?;
            socket.write_all(&[1u8]).await?;
            socket.flush().await?;

            let status = buffer[0];
            // 状态同步
            match status {
                0 => {
                    Store::new().set_status(Status::Closed);
                }
                1 => {
                    Store::new().set_status(Status::Open);
                }
                _ => {}
            }
        }
    }

    pub async fn stop() -> io::Result<()> {
        let port = Store::new().get_config().port;
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(&[0]).await?;

        stream.flush().await?;

        let mut buffer = [0; 1];
        stream.read(&mut buffer).await?;
        let status = buffer[0];
        match status {
            0 => {
                println!("poem exits with failure");
            }
            1 => {
                println!("poem exits successfully");
            }
            _ => {}
        }

        Ok(())
    }
}
