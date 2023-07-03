use crate::todo::model::Todo;
use actix_web::{delete, get, post, put, web, HttpResponse};

pub fn configure_todo_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(get)
            .service(get_single)
            .service(delete)
            .service(create)
            .service(update),
    );
}

#[get("")]
async fn get() -> HttpResponse {
    HttpResponse::Ok().body("Got all todos...")
}

#[get("{todo_id}")]
async fn get_single(todo_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Got todo with ID: {todo_id}"))
}

#[post("")]
async fn create(todo: web::Json<Todo>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "Creating todo...\nTitle: {}\ndescription: {}\nDue Date: {}",
        todo.title, todo.description, todo.due_date
    ))
}

#[put("{todo_id}")]
async fn update(todo_id: web::Path<u32>, todo: web::Json<Todo>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "Updating todo with ID: {todo_id}...\nTitle: {}\ndescription: {}\nDue Date: {}",
        todo.title, todo.description, todo.due_date
    ))
}

#[delete("{todo_id}")]
async fn delete(todo_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Deleting todo with ID: {todo_id}"))
}
