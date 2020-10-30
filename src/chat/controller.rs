use actix_identity::Identity;
use actix_web::{
    get,
    web::{Data, Path, Payload, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;

use std::fs;

use crate::chat::service::{create_room, ChatList, ChatRoom, get_sender, add_connection};
use crate::libs::time::get_current_time;
use crate::user::constant::Sign;

#[get("/ws/{receiver}")]
pub async fn websocket(
    Path(receiver): Path<String>,
    request: HttpRequest,
    stream: Payload,
    chat_list: Data<ChatList>,
    auth: Identity,
) -> impl Responder {
    if auth.identity().is_none() {
        return HttpResponse::Unauthorized().json(Sign::UNAUTHORIZED);
    }

    let sender = get_sender(&auth);
    let room = create_room(sender.to_owned(), receiver);
    let key = get_current_time();

    let chat_room = ws::start_with_addr(
        ChatRoom {
            clients: chat_list.addr.clone(),
            sender: sender,
            room: room.to_owned(),
            connection: key
        },
        &request,
        stream,
    );

    let (addr, response) = match chat_room {
        Ok(res) => res,
        Err(e) => return HttpResponse::from_error(e),
    };

    add_connection(chat_list.addr.clone(), addr, room, key);

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
