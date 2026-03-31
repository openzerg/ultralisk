pub struct HttpExecutor;

impl HttpExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HttpExecutor {
    fn default() -> Self {
        Self::new()
    }
}
