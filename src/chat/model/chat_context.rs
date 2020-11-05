use std::collections::HashMap;

use actix::Addr;

use super::chat_room::ChatRoom;

pub type ChatContext = HashMap<String, HashMap<u128, Addr<ChatRoom>>>;
