use crate::{
    handler::Handler,
    middleware::{Middleware, MiddlewareChain},
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
    middlewares: MiddlewareChain,
    router: Router,
}

impl ServerBuilder {
    pub async fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self {
            listener,
            middlewares: MiddlewareChain::new(),
            router: Router::new(),
        })
    }

    pub fn with<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(middleware);
        self
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
            middlewares: Arc::new(self.middlewares),
            router: Arc::new(self.router),
        }
    }
}

pub struct Server {
    listener: TcpListener,
    // Wrap with `Arc` to pass over tokio task without moving `self`.
    middlewares: Arc<MiddlewareChain>,
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
            let middlewares = Arc::clone(&self.middlewares);
            let router = Arc::clone(&self.router);
            tokio::spawn(async move {
                if let Ok(response) = Self::process(&mut stream, middlewares, router).await {
                    if let Err(err) = response.send(&mut stream).await {
                        eprintln!("{}", err);
                    }
                    stream.shutdown().await.unwrap();
                };
            });
        }
    }

    async fn process(
        stream: &mut TcpStream,
        middlewares: Arc<MiddlewareChain>,
        router: Arc<Router>,
    ) -> Result<Response, ()> {
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
        let request = match middlewares.run(request).await {
            Ok(request) => request,
            Err(code) => return Ok(code.respond_to()),
        };
        println!("{}", request);
        let handler = match router.find(request.uri()) {
            Some(handler) => handler,
            None => return Ok(StatusCode::NotFound.respond_to()),
        };
        let response = handler
            .call(request)
            .await
            .unwrap_or_else(|code| code.respond_to());
        Ok(response)
    }
}
