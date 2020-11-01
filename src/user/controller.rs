use actix_identity::Identity;
use actix_web::{
    get, post,
    web::ServiceConfig,
    web::{Data, Json},
    HttpResponse,
};

use sqlx::PgPool;

// use std::time::Duration;
// use tokio::time;

use std::fs::read_to_string;

use crate::libs::jwt;
use crate::libs::time::get_current_time;
use crate::models::api::{APIResponse, APIResult};
use crate::user::model::User;

use crate::user::service::Validation;

use crate::user::constant::{JWTError, Sign, ValidationError};

#[get("/signup")]
pub async fn client_sign_up() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(read_to_string("static/signup.html").expect("Sign up"))
}

#[get("/signin")]
pub async fn client_sign_in() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(read_to_string("static/signin.html").expect("Sign in"))
}

#[post("/signup")]
pub async fn sign_up(user: Json<User>, connection: Data<PgPool>) -> HttpResponse {
    if Validation::is_too_short(&user) {
        return HttpResponse::BadRequest().json(ValidationError::TOO_SHORT);
    }

    if Validation::is_too_long(&user) {
        return HttpResponse::BadRequest().json(ValidationError::TOO_LONG);
    }

    // time::delay_for(Duration::from_millis(250)).await;

    let user_existed: bool = user.exists(&connection).await.unwrap_or_else(|_| false);

    if user_existed {
        return HttpResponse::Unauthorized().json(Sign::EXISTED);
    }

    let sign_up = user.sign_up(&connection).await;

    match sign_up {
        Ok(_) => HttpResponse::Ok().json(APIResponse::SUCCESS),
        _ => HttpResponse::Unauthorized().json(APIResponse::FAILURE),
    }
}

#[post("/signin")]
pub async fn sign_in(user: Json<User>, connection: Data<PgPool>, auth: Identity) -> HttpResponse {
    if auth.identity().is_some() {
        return HttpResponse::BadRequest().json(Sign::SIGNED);
    }

    if Validation::is_too_short(&user) {
        return HttpResponse::BadRequest().json(ValidationError::TOO_SHORT);
    }

    if Validation::is_too_long(&user) {
        return HttpResponse::BadRequest().json(ValidationError::TOO_LONG);
    }

    // time::delay_for(Duration::from_millis(250)).await;

    let signed: bool = user.sign_in(&connection).await.unwrap_or_else(|_| false);

    if !signed {
        return HttpResponse::Unauthorized().json(Sign::INCORRECT);
    }

    let token = jwt::encode(&user.name);

    if token.is_none() {
        return HttpResponse::InternalServerError().json(JWTError::CREATION);
    }

    auth.remember(token.unwrap());

    HttpResponse::Ok().json(Sign::SUCCESS)
}

#[post("/refresh")]
pub fn refresh(auth: Identity) -> HttpResponse {
    if auth.identity().is_none() {
        return HttpResponse::Unauthorized().json(Sign::UNAUTHORIZED);
    }

    let token = jwt::decode(&auth.identity().unwrap());

    if token.is_none() {
        return HttpResponse::BadRequest().json(JWTError::INVALID);
    }

    let jwt_token = token.unwrap();

    if jwt_token.exp > get_current_time() + 86400 * 3 {
        return HttpResponse::Unauthorized().json(JWTError::EXPIRED);
    }

    let refresh_token = jwt::encode(&jwt_token.name);

    if refresh_token.is_none() {
        return HttpResponse::InternalServerError().json(JWTError::CREATION);
    }

    auth.remember(refresh_token.unwrap());

    HttpResponse::Ok().json(APIResult {
        success: true,
        detail: &jwt_token.name,
    })
}

#[get("/signout")]
pub fn client_sign_out(auth: Identity) -> HttpResponse {
    auth.forget();

    HttpResponse::Ok().json(Sign::OUT)
}

#[get("/signout")]
pub fn sign_out(auth: Identity) -> HttpResponse {
    auth.forget();

    HttpResponse::Ok().json(Sign::OUT)
}

pub fn user_module(config: &mut ServiceConfig) {
    config
        .service(client_sign_up)
        .service(client_sign_in)
        .service(client_sign_out);
    config
        .service(sign_in)
        .service(sign_up)
        .service(refresh)
        .service(sign_out);
}
