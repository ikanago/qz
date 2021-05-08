use crate::{
    handler::Handler,
    method::Method,
    middleware::{Middleware, MiddlewareChain},
    request::{ParseState, RequestBuffer},
    response::Response,
    router::Router,
    static_files::{StaticDir, StaticFile},
};
use std::{path::Path, sync::Arc};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Builder of `Server`.
/// The reason of this struct is to make `Server.router` immutable.
pub struct ServerBuilder<State>
where
    State: Clone + Send + Sync + 'static,
{
    listener: TcpListener,
    middlewares: MiddlewareChain,
    router: Router<State>,
    state: State,
}

impl ServerBuilder<()> {
    pub async fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self {
            listener,
            middlewares: MiddlewareChain::new(),
            router: Router::new(),
            state: (),
        })
    }
}

impl<State> ServerBuilder<State>
where
    State: Clone + Send + Sync + 'static,
{
    pub async fn with_state(port: u16, state: State) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self {
            listener,
            middlewares: MiddlewareChain::new(),
            router: Router::new(),
            state,
        })
    }

    pub fn with<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn route<F: Handler<State>>(mut self, path: &str, method: Method, handler: F) -> Self {
        self.router.add_route(path, method, handler);
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
        self.route(
            &serve_at_wildcard,
            Method::Get,
            StaticDir::mount(dir, serve_at),
        )
    }

    pub fn serve_file<P>(self, serve_at: &str, path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = StaticFile::mount(path)?;
        Ok(self.route(&serve_at, Method::Get, file))
    }

    pub fn build(self) -> Server<State> {
        Server {
            listener: self.listener,
            middlewares: Arc::new(self.middlewares),
            router: Arc::new(self.router),
            state: self.state,
        }
    }
}

pub struct Server<State>
where
    State: Clone + Send + Sync + 'static,
{
    listener: TcpListener,
    // Wrap with `Arc` to pass over tokio task without moving `self`.
    middlewares: Arc<MiddlewareChain>,
    router: Arc<Router<State>>,
    state: State,
}

impl<State> Server<State>
where
    State: Clone + Send + Sync + 'static,
{
    const INITIAL_BUFFER_SIZE: usize = 4096;

    pub async fn run(&self) -> io::Result<()> {
        println!("Listening on {}", self.listener.local_addr()?);
        loop {
            let (mut stream, _) = match self.listener.accept().await {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("{}", err);
                    break Ok(());
                }
            };
            let middlewares = Arc::clone(&self.middlewares);
            let router = Arc::clone(&self.router);
            let state = self.state.clone();
            tokio::spawn(async move {
                if let Ok(response) = Self::process(&mut stream, middlewares, router, state).await {
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
        router: Arc<Router<State>>,
        state: State,
    ) -> Result<Response, ()> {
        let mut request_buf = RequestBuffer::new();
        let mut buf = vec![0; Self::INITIAL_BUFFER_SIZE];
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => return Err(()),
                Ok(_) => match request_buf.try_parse(&buf) {
                    Ok(ParseState::Completed) => break,
                    Ok(_) => continue,
                    Err(code) => return Ok(Response::from(code)),
                },
                Err(_) => {
                    return Err(());
                }
            };
        }

        let request = request_buf.complete();
        let request = match middlewares.run(request).await {
            Ok(request) => request,
            Err(response) => return Ok(response),
        };
        println!("{}", request);
        let handler = match router.find(request.uri(), request.method()) {
            Ok(handler) => handler,
            Err(code) => return Ok(Response::from(code)),
        };
        let response = handler
            .call(request, state)
            .await
            .unwrap_or_else(|code| Response::from(code));
        Ok(response)
    }
}
