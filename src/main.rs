use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod models;
mod routes;

pub struct AppState {
    pub current_player_id: Mutex<Option<String>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Starting server...");
    let state = web::Data::new(AppState {
        current_player_id: Mutex::new(None),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // only allowed in dev
            .allowed_origin("https://decibel.esek.se")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allow_any_header();

        App::new().wrap(cors).app_data(state.clone()).service(
            web::scope("/mpris")
                .service(routes::mpris::get_current)
                .service(routes::mpris::get_players)
                .service(routes::mpris::get_player)
                .service(routes::mpris::play)
                .service(routes::mpris::pause)
                .service(routes::mpris::next)
                .service(routes::mpris::prev)
                .service(routes::websocket::index),
        )
        //.route("/ws/{id}", web::get().to(routes::websocket::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
