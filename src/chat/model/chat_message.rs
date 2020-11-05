use actix::{Handler, Message};

use anyhow::Result;
use sqlx::{FromRow, PgPool};

use super::chat_room::ChatRoom;

#[derive(Message, FromRow, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub r#type: String,
    pub sender: String,
    pub data: String,
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
            "msg", message.data, message.sender
        ));
    }
}
