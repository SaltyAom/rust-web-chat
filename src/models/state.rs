use std::sync::{Arc, Mutex};

use crate::chat::model::chat_context::ChatContext;

// ? Share State
pub struct ChatList {
    pub addr: Arc<Mutex<ChatContext>>,
}
