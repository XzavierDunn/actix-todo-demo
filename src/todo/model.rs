use serde::Deserialize;

#[derive(Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub due_date: String,
}
