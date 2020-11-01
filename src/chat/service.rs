use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix::Addr;
use actix_identity::Identity;

use crate::chat::model::{ChatContext, ChatRoom};

use crate::libs::jwt::decode;

pub fn create_room(sender: String, receiver: String) -> String {
    if sender.as_bytes() < receiver.as_bytes() {
        sender + "_" + &receiver
    } else {
        receiver + "_" + &sender
    }
}

pub fn get_sender(auth: &Identity) -> String {
    let token = auth.identity().unwrap();
    let jwt_token = decode(&token).unwrap();

    jwt_token.name
}

pub fn add_connection(
    address: Arc<Mutex<ChatContext>>,
    addr: Addr<ChatRoom>,
    room: String,
    key: u128,
) {
    let mut address = address.lock().unwrap();

    if !address.contains_key(&room) {
        address.insert(room.to_owned(), HashMap::new());
    }

    let connection = address.get_mut(&room).unwrap();
    connection.insert(key, addr);
}
