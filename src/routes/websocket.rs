use std::time::Duration;

use actix::{Actor, AsyncContext, SpawnHandle, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::routes;

struct MyWs {
    player: mpris::Player<'static>,
    is_subscribed: bool,
    handle: Option<SpawnHandle>,
}

impl MyWs {
    fn toggle_sub(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        match self.is_subscribed {
            false => self.sub(ctx),
            true => self.unsub(ctx),
        }
    }

    fn sub(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.is_subscribed = true;
        if self.handle.is_some() {
            return;
        }
        self.handle = Some(ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            let json =
                serde_json::to_string(&crate::models::player::Player::from_mpris(&act.player))
                    .unwrap();
            ctx.text(json);
        }));
    }

    fn unsub(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.is_subscribed = false;
        if self.handle.is_none() {
            return;
        }
        ctx.cancel_future(self.handle.take().unwrap());
        self.handle = None;
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn started(&mut self, ctx: &mut Self::Context) {
        let json = serde_json::to_string(&crate::models::player::Player::from_mpris(&self.player))
            .unwrap();
        ctx.text(json);
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match text.trim() {
                    "sub" => self.toggle_sub(ctx),
                    "next" => self.player.next().unwrap(),
                    "prev" => self.player.previous().unwrap(),
                    "pause" => self.player.pause().unwrap(),
                    "play" => self.player.play().unwrap(),
                    &_ => ctx.text("unknown command"),
                };
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

// Websocket connections get upgraded after get requests
#[get("/{id}/ws")]
pub async fn index(
    id: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        MyWs {
            player: routes::mpris::get_player_by_id(&id).unwrap(),
            is_subscribed: false,
            handle: None,
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}
