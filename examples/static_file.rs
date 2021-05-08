use qz::server::Server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder()
        .serve_file("/pages", "./examples/assets/index.html")?
        .build();
    Server::run(server, 8080).await
}
