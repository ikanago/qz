use crate::{
    handler::Handler,
    method::Method,
    middleware::{Middleware, MiddlewareChain},
    request::{ParseState, Request, RequestBuffer},
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
    middlewares: Vec<Arc<dyn Middleware<State>>>,
    router: Router<State>,
    state: State,
}

impl ServerBuilder<()> {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            router: Router::new(),
            state: (),
        }
    }
}

impl<State> ServerBuilder<State>
where
    State: Clone + Send + Sync + 'static,
{
    pub fn with_state(state: State) -> Self {
        Self {
            middlewares: Vec::new(),
            router: Router::new(),
            state,
        }
    }

    pub fn with<M: Middleware<State>>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
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
            middlewares: Arc::new(self.middlewares),
            router: Arc::new(self.router),
            state: self.state,
        }
    }
}

#[derive(Clone)]
pub struct Server<State>
where
    State: Clone + Send + Sync + 'static,
{
    // Wrap with `Arc` to pass over tokio task without moving `self`.
    middlewares: Arc<Vec<Arc<dyn Middleware<State>>>>,
    router: Arc<Router<State>>,
    state: State,
}

impl Server<()> {
    pub fn builder() -> ServerBuilder<()> {
        ServerBuilder::new()
    }
}

impl<State> Server<State>
where
    State: Clone + Send + Sync + 'static,
{
    const INITIAL_BUFFER_SIZE: usize = 4096;

    pub fn builder_with_state(state: State) -> ServerBuilder<State> {
        ServerBuilder::with_state(state)
    }

    pub async fn run(server: Self, port: u16) -> io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        println!("Listening on {}", listener.local_addr()?);
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("{}", err);
                    break Ok(());
                }
            };

            let server = server.clone();
            tokio::spawn(async move {
                if let Ok(response) = server.process(&mut stream).await {
                    if let Err(err) = response.send(&mut stream).await {
                        eprintln!("{}", err);
                    }
                    stream.shutdown().await.unwrap();
                };
            });
        }
    }

    async fn process(self, stream: &mut TcpStream) -> Result<Response, ()> {
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
        let response = self.respond(request).await;
        Ok(response)
    }

    pub(crate) async fn respond(self, request: Request) -> Response {
        let Server {
            middlewares,
            router,
            state,
        } = self;

        println!("{}", request);
        let handler = match router.find(request.uri(), request.method()) {
            Ok(handler) => handler,
            Err(code) => return Response::from(code),
        };
        let chain = MiddlewareChain {
            handler: handler.as_ref(),
            middlewares: &middlewares,
        };
        chain.run(request, state).await
    }
}
