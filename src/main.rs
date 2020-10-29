mod user;
mod libs;
mod ws;
mod models;
mod chat;

use actix_web::{ HttpServer, App, middleware::Compress };
use actix_identity::{CookieIdentityPolicy, IdentityService};

use sqlx::postgres::PgPoolOptions;

use cookie::SameSite;

use dotenv::dotenv;
use anyhow::Result;

use std::env;

use user::controller::user_module;
use chat::controller::chat_module;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL");
    let connection_pool = PgPoolOptions::new().connect(&database_url).await?;
        
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
            .configure(user_module)
            .configure(chat_module)
    )
    .bind("localhost:8080")?
    .run()
    .await
    .ok();

    Ok(())
}