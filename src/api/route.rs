use actix_files::NamedFile;
use actix_web::{Responder, http::header::ContentType, HttpResponse};
use actix_web::Result;

/// favicon handler
pub async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

/// Index Page
pub async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open("./static/index.html")?)
}

/// Button Clicked
pub async fn clicked() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/test.html")))
}
