use crate::libs::hash::hash;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pass: String,
}

impl User {
    pub async fn sign_up(&self, connection: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users(name, pass) VALUES($1, $2)
        "#,
            self.name,
            hash(&self.pass)
        )
        .execute(connection)
        .await?;

        Ok(())
    }

    pub async fn exists(&self, connection: &PgPool) -> Result<bool> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count FROM users WHERE name = $1
        "#,
        )
        .bind(self.name.to_owned())
        .fetch_one(connection)
        .await?;

        Ok(count == 1)
    }

    pub async fn sign_in(&self, connection: &PgPool) -> Result<bool> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count FROM users WHERE name = $1 AND pass = $2
        "#,
        )
        .bind(self.name.to_owned())
        .bind(hash(&self.pass))
        .fetch_one(connection)
        .await?;

        Ok(count == 1)
    }
}
