use crate::{
    handler::Handler,
    request::{ParseState, RequestBuffer},
    response::Response,
    router::Router,
    status::StatusCode,
};
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct ServerBuilder {
    listener: TcpListener,
    router: Router,
}

impl ServerBuilder {
    pub async fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let router = Router::new();
        Ok(Self { listener, router })
    }

    pub fn route<F: Handler>(mut self, path: &str, handler: F) -> Self {
        self.router.add_route(path.as_bytes(), handler);
        self
    }

    pub fn build(self) -> Server {
        Server {
            listener: self.listener,
            router: Arc::new(self.router),
        }
    }
}

pub struct Server {
    listener: TcpListener,
    // Wrap with `Arc` to pass over tokio task without moving `self`.
    router: Arc<Router>,
}

impl Server {
    const INITIAL_BUFFER_SIZE: usize = 8;

    pub async fn run(&self) -> io::Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let router = Arc::clone(&self.router);
            tokio::spawn(async move {
                if let Ok(response) = Self::process(&mut stream, router).await {
                    if let Err(err) = response.send(&mut stream).await {
                        eprintln!("{}", err);
                    }
                    stream.shutdown().await.unwrap();
                };
            });
        }
    }

    async fn process(stream: &mut TcpStream, router: Arc<Router>) -> Result<Response, ()> {
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
                    return Ok(Response::from(StatusCode::NotFound));
                }
            };
        }

        let request = request_buf.complete();
        println!("{}", request);
        let handler = match router.find(request.uri().as_ref()) {
            Some(handler) => handler,
            None => return Ok(Response::from(StatusCode::NotFound)),
        };
        let response = handler.call(request);
        return Ok(response);
    }
}
