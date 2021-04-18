extern crate qz;

use qz::server::Server;

#[tokio::main]
async fn main() {
    let port = 8080;
    let server = Server::new(port).await.unwrap();
    server.run().await.unwrap();
}
