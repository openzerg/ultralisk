pub struct MessageBus;

impl MessageBus {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
