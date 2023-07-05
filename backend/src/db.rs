use std::sync::{Arc, Mutex};
use crate::question::Question;

#[derive(Clone, Default)]
pub struct AppDatabase {
    pub questions: Arc<Mutex<Vec<Question>>>,
}

