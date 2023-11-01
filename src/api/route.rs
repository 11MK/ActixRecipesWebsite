use std::collections::HashMap;

use actix_files::NamedFile;
use actix_web::{ HttpResponse, Responder};
use actix_web::{web, Result};

/// favicon handler
pub async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

/// Index Page
pub async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open("./static/index.html")?)
}

/// Test Templates & Htmx
pub async fn test(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    if query.get("name").is_none() {
        let body = tmpl.render("default.html", &tera::Context::new()).unwrap();
        Ok(HttpResponse::Ok().body(body))
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("name", query.get("name").unwrap());
        ctx.insert("text", "Welcome!");
        let body = tmpl.render("test.html", &ctx).unwrap();
        Ok(HttpResponse::Ok().body(body))
    }
}

/// Button Clicked
pub async fn clicked() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/test.html")))
}
