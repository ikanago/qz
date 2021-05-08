use qz::server::Server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder().serve_dir("/", "./target/doc").build();
    Server::run(server, 8080).await
}
