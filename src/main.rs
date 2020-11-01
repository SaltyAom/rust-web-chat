mod user;
mod libs;
mod models;
mod chat;

use actix_web::{ HttpServer, App, middleware::Compress, web::Data };
use actix_identity::{CookieIdentityPolicy, IdentityService};

use sqlx::postgres::PgPoolOptions;

use cookie::SameSite;

use dotenv::dotenv;
use anyhow::Result;

use std::env;
use std::sync::{ Mutex, Arc };
use std::collections::HashMap;

use user::controller::user_module;
use chat::controller::chat_module;
use chat::model::ChatList;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL");
    let connection_pool = PgPoolOptions::new().connect(&database_url).await?;

    let chat_state = Data::new(ChatList { addr: Arc::new(Mutex::new(HashMap::new())) });

    HttpServer::new(move ||
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(
                    env::var("cookie_secret").expect("Cookie Secret").as_bytes()
                )
                    .name("auth-cookie")
                    .same_site(SameSite::Strict)
                    .secure(false)
                    .max_age(86400 * 3)
                )
            )
            .wrap(Compress::default())
            .data(connection_pool.clone())
            .app_data(chat_state.clone())
            .configure(user_module)
            .configure(chat_module)
    )
    .bind("localhost:8080")?
    .run()
    .await
    .ok();

    Ok(())
}