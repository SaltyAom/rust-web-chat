use actix_web::web::Json;

use crate::user::model::User;

pub struct Validation;

impl Validation {
    pub fn is_too_short(user: &Json<User>) -> bool {
        user.name.trim().len() < 5 || user.pass.trim().len() < 5
    }

    pub fn is_too_long(user: &Json<User>) -> bool {
        user.name.trim().len() > 32 || user.pass.trim().len() > 32
    }
}
