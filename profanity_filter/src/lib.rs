pub struct Message {
   pub content: String,
   pub user: String,
}

impl Message {
    pub fn new(content: &str, user: &str) -> Self {
        Self {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    pub fn send_ms(self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<String, &str> {
    let msg = Message::new(message, "anonymous");
    match msg.send_ms() {
        Some(content) => Ok(content),
        None => Err("ERROR: illegal"),
    }
}
