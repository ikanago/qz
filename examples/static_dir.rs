use qz::server::ServerBuilder;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .serve_dir("/pages", "./examples/assets")
        .build()
        .run()
        .await
}
