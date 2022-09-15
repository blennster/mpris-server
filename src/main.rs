use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // only allowed in dev
            .allowed_origin("https://decibel.esek.se")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allow_any_header();

        App::new().wrap(cors).service(routes::mpris::players)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
