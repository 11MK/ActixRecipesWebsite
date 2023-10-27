use actix_web::{
    http::{self, Method, StatusCode},
    middleware::{self, ErrorHandlers},
    web, App, Either, HttpResponse, HttpServer, Responder, Result,
};

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 8080;
    log::info!("starting HTTP server at http://localhost:{}", port);

    HttpServer::new(|| {
        let error_handlers = ErrorHandlers::new()
            .handler(http::StatusCode::INTERNAL_SERVER_ERROR, api::error::internal_server_error)
            .handler(http::StatusCode::BAD_REQUEST, api::error::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::error::not_found);
        App::new()
            .wrap(error_handlers)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(api::route::index)))
            .service(web::resource("/favicon").route(web::get().to(api::route::favicon)))
            .service(web::resource("/clicked").route(web::get().to(api::route::clicked)))
            // register favicon
            // .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
