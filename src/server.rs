use crate::request::{ParseState, RequestBuffer};
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
        let mut request_buf = RequestBuffer::new();
        let mut buf = vec![0; Self::INITIAL_BUFFER_SIZE];
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => return,
                Ok(_) => match request_buf.try_parse(&buf) {
                    Ok(ParseState::Completed) => break,
                    Ok(_) => continue,
                    Err(_) => return,
                },
                _ => return,
            };
        }
        let request = request_buf.complete();

        dbg!(request);
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).await.unwrap();
        stream.shutdown().await.unwrap();
    }
}
