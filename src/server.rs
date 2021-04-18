use crate::request::Request;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    const INITIAL_BUFFER_SIZE: usize = 8;

    pub async fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Self { listener })
    }

    pub async fn run(&self) -> io::Result<()> {
        loop {
            let (stream, _) = self.listener.accept().await?;
            tokio::spawn(async move {
                Self::process(stream).await;
            });
        }
    }

    pub async fn process(mut stream: TcpStream) {
        let mut received: Vec<u8> = Vec::with_capacity(Self::INITIAL_BUFFER_SIZE);
        let mut buf = vec![0; Self::INITIAL_BUFFER_SIZE];
        // Even if client stop sending data, stream.read() never returns 0, so stucks.
        // Read data and check if it contains '\r' by iter::rposition(), then parse request header.
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    dbg!(n);
                    received.extend(&buf);
                }
                _ => return,
            };
        }
        dbg!(&received);

        match Request::new(&received) {
            Ok(req) => {
                dbg!(req);
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).await.unwrap();
            }
            Err(err) => {
                dbg!(err);
                stream
                    .write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")
                    .await
                    .unwrap();
                return;
            }
        }
        stream.flush().await.unwrap();
    }
}
