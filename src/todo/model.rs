use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub due_date: String,
}
