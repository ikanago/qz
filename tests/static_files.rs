use qz::{
    body::Body,
    handler::Handler,
    header::HeaderName,
    request::RequestBuffer,
    static_files::{StaticDir, StaticFile},
    status::StatusCode,
};
use std::collections::HashMap;
use std::io;

#[tokio::test]
async fn static_file() -> io::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /index.html HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticFile::mount("./tests/index.html")?;
    let response = handler.call(request).await;
    assert_eq!(StatusCode::Ok, response.status_code());
    assert_eq!(
        &vec![(HeaderName::ContentLength, b"13".to_vec())]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        response.headers()
    );
    assert_eq!(&Body::from(&b"<p>Hello</p>\n"[..]), response.body());
    Ok(())
}

#[tokio::test]
async fn static_dir() -> io::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /assets/index.html HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticDir::mount("./tests/assets", "/assets");
    let response = handler.call(request).await;
    assert_eq!(StatusCode::Ok, response.status_code());
    assert_eq!(
        &vec![
            (HeaderName::ContentLength, b"13".to_vec()),
            (HeaderName::ContentType, b"text/html".to_vec())
        ]
        .into_iter()
        .collect::<HashMap<_, _>>(),
        response.headers()
    );
    assert_eq!(&Body::from(&b"<p>Hello</p>\n"[..]), response.body());
    Ok(())
}

#[tokio::test]
async fn static_dir_not_found() -> io::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /assets/index.css HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticDir::mount("./tests/assets", "/assets");
    let response = handler.call(request).await;
    assert_eq!(StatusCode::NotFound, response.status_code());
    Ok(())
}
