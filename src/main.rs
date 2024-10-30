use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Cookie, Key, SameSite},
    error::InternalError,
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use mongodb::Client;

mod handlers;
pub mod model;

const REDIS_ADDRESS: &str = "redis://127.0.0.1:6379";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    
    let sessions_key = Key::generate();

    let session_storage = RedisSessionStore::new(REDIS_ADDRESS).await.expect("Redis configuration");

    let mongodb_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let mongodb_client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");
    
    tracing::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            // Logger
            .wrap(middleware::Logger::default())
            // Hello World (will be the general api information page)
            .service(handlers::hello)
            // cookie session
            .wrap(
                SessionMiddleware::builder(session_storage.clone(), sessions_key.clone())
                    // allow the cookie to be accessed from javascript
                    .cookie_http_only(false)
                    // allow the cookie only from the current domain
                    .cookie_same_site(SameSite::Strict)
                    .build(),
            )
            .service(handlers::login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}