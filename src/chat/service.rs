use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_web_actors::ws;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ? Share State
pub struct ChatList {
    pub addr: Arc<Mutex<HashMap<String, Addr<ChatRoom>>>>,
}

pub struct ChatRoom {
    pub clients: Arc<Mutex<HashMap<String, Addr<ChatRoom>>>>,
    pub key: Option<String>,
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
                let receiver = reverse_key(self.key.to_owned().unwrap());

                if clients.contains_key(&receiver) {
                    let receiver_client = clients.get(&receiver).unwrap();

                    receiver_client.do_send(ChatMessage {
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

        clients.remove(self.key.as_ref().unwrap());

        println!("{} disconnected", self.key.as_ref().unwrap());
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct Identifier {
    pub key: String,
}

impl Handler<Identifier> for ChatRoom {
    type Result = ();

    fn handle(&mut self, identifier: Identifier, _ctx: &mut Self::Context) {
        self.key = Some(identifier.key);
    }
}

pub fn create_key(sender: String, receiver: String) -> String {
    sender + "-" + &receiver
}

pub fn reverse_key(key: String) -> String {
    let k: Vec<&str> = key.split("-").collect();

    k[1].to_owned() + "-" + k[0]
}
