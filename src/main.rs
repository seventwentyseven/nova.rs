mod api;
mod config;
mod constants;
mod objects;
mod state;
mod utils;

use actix_web::{App, HttpServer};
use env_logger::Env;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(|| {
        App::new()
            .service(api::domains::ava_handler())
            .service(api::domains::cho_handler())
            .service(api::domains::osu_handler())
            // copy cho_handler to c4-6.dzifors.pl
            // TODO: Somehow add request method
            // TODO2: Depending on env, use different logger
            .wrap(actix_web::middleware::Logger::new(
                "%{CF-Connecting-IP}i | %{host}i%U %s | Time Taken: %D",
            ))
        // prod:
        // .wrap(actix_web::middleware::Logger::default())
    });

    println!("Running on: {}", config::CONFIG.General.host);
    // TODO call config init before start
    server.bind(&config::CONFIG.General.host)?.run().await
}
