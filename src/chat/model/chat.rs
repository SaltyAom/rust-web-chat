use anyhow::Result;
use sqlx::{FromRow, PgPool};

use serde::Serialize;

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
