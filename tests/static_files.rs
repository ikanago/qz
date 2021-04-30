use qz::{
    body::Body,
    handler::Handler,
    header::HeaderName,
    request::RequestBuffer,
    static_files::{StaticDir, StaticFile},
    status::StatusCode,
};

#[tokio::test]
async fn static_file() -> qz::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /index.html HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticFile::mount("./tests/index.html")?;
    let response = handler.call(request).await?;
    assert_eq!(StatusCode::Ok, response.status_code());
    assert_eq!(
        Some(&b"13".to_vec()),
        response.get_header(&HeaderName::ContentLength)
    );
    assert_eq!(
        Some(&b"text/html".to_vec()),
        response.get_header(&HeaderName::ContentType)
    );
    assert_eq!(&Body::from(&b"<p>Hello</p>\n"[..]), response.body());
    Ok(())
}

#[tokio::test]
async fn static_dir() -> qz::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /assets/index.html HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticDir::mount("./tests/assets", "/assets");
    let response = handler.call(request).await?;
    assert_eq!(StatusCode::Ok, response.status_code());
    assert_eq!(
        Some(&b"13".to_vec()),
        response.get_header(&HeaderName::ContentLength)
    );
    assert_eq!(
        Some(&b"text/html".to_vec()),
        response.get_header(&HeaderName::ContentType)
    );
    assert_eq!(&Body::from(&b"<p>Hello</p>\n"[..]), response.body());
    Ok(())
}

#[tokio::test]
async fn static_dir_not_found() -> qz::Result<()> {
    let mut buffer = RequestBuffer::new();
    buffer
        .try_parse(b"GET /assets/index.css HTTP/1.1\r\n\r\n")
        .unwrap();
    let request = buffer.complete();

    let handler = StaticDir::mount("./tests/assets", "/assets");
    assert_eq!(Err(StatusCode::NotFound), handler.call(request).await);
    Ok(())
}
