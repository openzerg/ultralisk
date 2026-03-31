use crate::core::interfaces::Storage;
use std::sync::Arc;

pub struct FileTime {
    storage: Arc<dyn Storage>,
}

impl FileTime {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
}
