use crate::{response::Response,request::{ParseState, RequestBuffer}};
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
            let (mut stream, _) = self.listener.accept().await?;
            tokio::spawn(async move {
                if let Ok(response) = Self::process(&mut stream).await {
                    stream.write_all(response.as_bytes()).await.unwrap_or(());
                    stream.shutdown().await.unwrap();
                };
            });
        }
    }

    pub async fn process(stream: &mut TcpStream) -> Result<String, ()> {
        let mut request_buf = RequestBuffer::new();
        let mut buf = vec![0; Self::INITIAL_BUFFER_SIZE];
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => return Err(()),
                Ok(_) => match request_buf.try_parse(&buf) {
                    Ok(ParseState::Completed) => break,
                    Ok(_) => continue,
                    Err(_) => return Err(()),
                },
                Err(_) => {
                    return Ok("HTTP/1.1 404 OK\r\n\r\n".to_string());
                }
            };
        }
        let request = request_buf.complete();

        println!("{}", request);
        let response = Response::default();
        return Ok(response.into_bytes());
    }
}
