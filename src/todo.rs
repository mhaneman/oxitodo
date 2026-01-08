use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }

    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
