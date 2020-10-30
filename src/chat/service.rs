use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_identity::Identity;
use actix_web_actors::ws;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::libs::jwt::decode;

type ChatContext = HashMap<String, HashMap<u128, Addr<ChatRoom>>>;

// ? Share State
pub struct ChatList {
    pub addr: Arc<Mutex<ChatContext>>,
}

pub struct ChatRoom {
    pub clients: Arc<Mutex<ChatContext>>,
    pub room: String,
    pub connection: u128,
    pub sender: String,
}

impl Actor for ChatRoom {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatRoom {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let clients = self.clients.lock().unwrap();

                if !clients.contains_key(&self.room) {
                    return;
                }

                let receivers = clients.get(&self.room).unwrap();

                for (receiver_connection, receiver) in receivers.iter() {
                    if receiver_connection == &self.connection {
                        return;
                    }

                    receiver.do_send(ChatMessage {
                        data: format!(r#"["{}","{}","{}"]"#, "msg", text, self.sender),
                    });
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        let mut clients = self.clients.lock().unwrap();

        let receivers = clients.get_mut(&self.room).unwrap();
        receivers.remove(&self.connection);

        if receivers.len() == 0 {
            clients.remove(&self.room);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage {
    data: String,
}

impl Handler<ChatMessage> for ChatRoom {
    type Result = ();

    fn handle(&mut self, message: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(message.data);
    }
}

pub fn create_room(sender: String, receiver: String) -> String {
    if sender.as_bytes() < receiver.as_bytes() {
        sender + "-" + &receiver
    } else {
        receiver + "-" + &sender
    }
}

pub fn get_sender(auth: &Identity) -> String {
    let token = auth.identity().unwrap();
    let jwt_token = decode(&token).unwrap();

    jwt_token.name
}

pub fn add_connection(
    address: Arc<Mutex<ChatContext>>,
    addr: Addr<ChatRoom>,
    room: String,
    key: u128,
) {
    let mut address = address.lock().unwrap();

    if !address.contains_key(&room) {
        address.insert(room.to_owned(), HashMap::new());
    }

    let connection = address.get_mut(&room).unwrap();
    connection.insert(key, addr);
}
