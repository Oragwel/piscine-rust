pub struct Message {
    pub content: String,
    pub user: String,
    }
    
    impl Message {
    pub fn new(content: String) -> Self {
    Self {
    content,
    user: String::new(),
    }
    }
    
    pub fn send_ms(&self) -> Option<String> {
    if self.content.is_empty() || self.content.contains("stupid") {
    None
    } else {
    Some(self.content.clone())
    }
    }
    }
    
    pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string());
    match msg.send_ms() {
    Some(_) => Ok(message),
    None => Err("ERROR: illegal"),
    }
    }