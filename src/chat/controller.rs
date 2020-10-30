use actix_identity::Identity;
use actix_web::{
    get,
    web::{Data, Path, Payload, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;

use std::fs;

use crate::chat::service::{create_key, ChatList, ChatRoom, Identifier};
use crate::libs::jwt::decode;
use crate::user::constant::Sign;

#[get("/ws/{receiver}")]
pub async fn websocket(
    Path(receiver): Path<String>,
    request: HttpRequest,
    stream: Payload,
    chatroom: Data<ChatList>,
    auth: Identity,
) -> impl Responder {
    if auth.identity().is_none() {
        return HttpResponse::Unauthorized().json(Sign::UNAUTHORIZED);
    }

    let token = auth.identity().unwrap();
    let jwt_token = decode(&token).unwrap();
    let sender = jwt_token.name;

    let key = create_key(sender.to_owned(), receiver);

    let chat_room = ws::start_with_addr(
        ChatRoom {
            clients: chatroom.addr.clone(),
            sender: sender,
            key: None,
        },
        &request,
        stream,
    );

    let (addr, response) = match chat_room {
        Ok(res) => res,
        Err(e) => return HttpResponse::from_error(e),
    };

    addr.do_send(Identifier {
        key: key.to_owned(),
    });

    let mut addresses = chatroom.addr.lock().unwrap();
    if !addresses.contains_key(&key) {
        addresses.insert(key, addr);
    }

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
