use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    
    tracing::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn hello() -> Result<impl Responder, Error> {
    Ok("World!")
}