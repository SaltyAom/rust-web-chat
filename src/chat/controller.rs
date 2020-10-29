use actix_web::{
    get,
    web::{Payload, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use std::fs;

use crate::ws::WebSocket;

#[get("/ws")]
pub async fn websocket(request: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    let response = ws::start(WebSocket {}, &request, stream);

    response
}

#[get("/")]
pub async fn client() -> HttpResponse {
    let html = fs::read_to_string("static/index.html").expect("index.html");

    HttpResponse::Ok().content_type("text/html").body(html)
}

pub fn chat_module(config: &mut ServiceConfig) {
    config.service(client).service(websocket);
}
