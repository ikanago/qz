use crate::{
    handler::Handler,
    request::{ParseState, RequestBuffer},
    response::{Responder, Response},
    router::Router,
    static_files::{StaticDir, StaticFile},
    status::StatusCode,
};
use std::{path::Path, sync::Arc};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Builder of `Server`.
/// The reason of this struct is to make `Server.router` immutable.
pub struct ServerBuilder {
    listener: TcpListener,
    router: Router,
}

impl ServerBuilder {
    pub async fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        let router = Router::new();
        Ok(Self { listener, router })
    }

    pub fn route<F: Handler>(mut self, path: &str, handler: F) -> Self {
        self.router.add_route(path, handler);
        self
    }

    /// Serve files under the directory.
    /// `dir` is path to the directory and `serve_at` is a prefix of URI.
    /// e.g. `self.serve_dir("./static/html", /static)` serves files under `./static/html` and
    /// URI for the files will be like `/static/index.html`
    pub fn serve_dir<P>(self, serve_at: &str, dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut serve_at_wildcard = serve_at.trim_end_matches('/').to_string();
        serve_at_wildcard.push_str("/*");
        self.route(&serve_at_wildcard, StaticDir::mount(dir, serve_at))
    }

    pub fn serve_file<P>(self, serve_at: &str, path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = StaticFile::mount(path)?;
        Ok(self.route(&serve_at, file))
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
    const INITIAL_BUFFER_SIZE: usize = 1024;

    pub async fn run(&self) -> io::Result<()> {
        println!("Listening on {}", self.listener.local_addr()?);
        loop {
            let (mut stream, _) = match self.listener.accept().await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("{}", e);
                    break Ok(());
                }
            };
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
                    Err(code) => return Ok(code.respond_to()),
                },
                Err(_) => {
                    return Err(());
                }
            };
        }

        let request = request_buf.complete();
        println!("{}", request);
        let handler = match router.find(request.uri()) {
            Some(handler) => handler,
            None => return Ok(StatusCode::NotFound.respond_to()),
        };
        let response = match handler.call(request).await {
            Ok(response) => response,
            Err(code) => code.respond_to(),
        };
        Ok(response)
    }
}
