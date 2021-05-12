//! Lupt chat
//! Chat Website to have group chat and stranger's chat both
//! 
//! Structure of how program work flow
//!
//!           |--> ws_sansad1 <----\
//! ws_index -|--> ws_sansad2 <---- \ chat_pind
//!           |--> ws_sansad3 <---- /
//!           |--> ws_sansad4 <----/
//!

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware::Logger, web};
use actix_files as fs;
use actix_web_actors::ws;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use ws_sansad::WsSansad;

mod config;
mod errors;
mod broker_messages;
mod ws_sansad;
mod chat_pinnd;
mod validator;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let store = MemoryStore::new();
    let config = config::Config::new();
    let static_path = config.static_path;
    HttpServer::new(move || {
        App::new()
        .wrap(
            RateLimiter::new(
            MemoryStoreActor::from(store.clone()).start())
                .with_interval(std::time::Duration::from_secs(60))
                .with_max_requests(200)
        )
        .wrap(Logger::new("%t [%{x-forwarded-for}i] %s %{User-Agent}i %r"))
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", &static_path).index_file("index.html"))
    })
    .bind(config.bind_address)?
    .run()
    .await
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSansad::new(), &req, stream)
}
