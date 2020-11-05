use actix::{Actor, Arbiter, StreamHandler};
use actix_web_actors::ws;

use sqlx::PgPool;
use std::sync::{Arc, Mutex};

use super::chat_context::ChatContext;
use super::chat_message::ChatMessage;

pub struct ChatRoom {
    pub clients: Arc<Mutex<ChatContext>>,
    pub room: String,
    pub connection: u128,
    pub sender: String,
    pub database_connection: PgPool,
}

impl Actor for ChatRoom {
    type Context = ws::WebsocketContext<Self>;
}

impl ChatRoom {
    async fn save_message_to_database(
        chat_message: ChatMessage,
        room: String,
        database_connection: PgPool,
    ) {
        let message = chat_message.save(room, database_connection).await;

        match message {
            Ok(data) => data,
            Err(_) => (),
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatRoom {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let clients = self.clients.lock().unwrap();

                // ? If user exit before chat message reached, ignore message
                if !clients.contains_key(&self.room) || text.len() == 0 {
                    return;
                }

                let receivers = clients.get(&self.room).unwrap();
                let chat_message = ChatMessage {
                    r#type: "msg".to_owned(),
                    data: text.to_owned(),
                    sender: self.sender.to_owned(),
                };

                // ? Only self is listening, ignore sent message
                if receivers.len() > 1 {
                    for (receiver_connection, receiver) in receivers.iter() {
                        if receiver_connection != &self.connection {
                            receiver
                                .try_send(chat_message.clone())
                                .expect("Actor queue");
                        }
                    }
                }

                // ? Takeover lifetime due to move into future.
                let room = self.room.to_owned();
                let future_database_connection = self.database_connection.to_owned();

                let save_message_to_database =
                    Self::save_message_to_database(chat_message, room, future_database_connection);

                Arbiter::spawn(save_message_to_database);
            }
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
