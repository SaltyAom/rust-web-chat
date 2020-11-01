use actix::{Actor, Addr, Handler, Message, StreamHandler, Arbiter};
use actix_web_actors::ws;

use anyhow::Result;
use sqlx::{FromRow, PgPool};

use serde::Serialize;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type ChatContext = HashMap<String, HashMap<u128, Addr<ChatRoom>>>;

// ? Share State
pub struct ChatList {
    pub addr: Arc<Mutex<ChatContext>>,
}

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

                // ? In some weird bug case where room is empty, ignore message
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
                            receiver.do_send(chat_message.clone());
                        }
                    }
                }

                // ? Takeover lifetime due to move into future.
                let room = self.room.to_owned();
                let future_database_connection = self.database_connection.to_owned();

                let save_message_to_database = async move {
                    let message = chat_message.save(room, future_database_connection).await;
                    
                    match message {
                        Ok(data) => data,
                        Err(_) => ()
                    }
                };

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

#[derive(Message, FromRow, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
    r#type: String,
    sender: String,
    data: String,
}

impl ChatMessage {
    pub async fn save(&self, room: String, connection: PgPool) -> Result<()> {
        // println!("Save: {}", self.data);

        sqlx::query!(
            r#"
            INSERT INTO message(key, type, data, sender) VALUES($1, $2, $3, $4)
            "#,
            room,
            self.r#type.to_owned(),
            self.data.to_owned(),
            self.sender.to_owned()
        )
        .execute(&connection)
        .await?;

        Ok(())
    }
}

impl Handler<ChatMessage> for ChatRoom {
    type Result = ();

    fn handle(&mut self, message: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(format!(
            r#"["{}","{}","{}"]"#,
            "msg", message.data, self.sender
        ));
    }
}

#[derive(FromRow, Serialize)]
pub struct Chat {
    pub r#type: String,
    pub sender: String,
    pub data: String,
    pub time: f64,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            r#type: "message".to_owned(),
            sender: "".to_owned(),
            data: "".to_owned(),
            time: 0.0,
        }
    }
}

impl Chat {
    pub async fn history(
        &self,
        key: String,
        pagination: i64,
        connection: &PgPool,
    ) -> Result<Vec<Chat>> {
        // println!("Load {}", key);

        let history = sqlx::query_as::<_, Chat>(
            r#"SELECT type, data, sender, extract(epoch from time) AS time FROM message WHERE key = $1 ORDER BY time DESC LIMIT 50 OFFSET $2"#
        )
        .bind(key.to_owned())
        .bind(pagination - 1)
        .fetch_all(connection)
        .await?;

        Ok(history)
    }
}
