use crate::dynamo::crud::{
    create_record, delete_record, get_records, get_single_record, update_record,
};
use crate::todo::model::Todo;
use actix_web::{delete, get, post, put, web, HttpResponse};

use super::model::TodoItem;

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
    match get_records().await {
        Ok(items) => HttpResponse::Ok().json(items),

        Err(e) => {
            println!("{e}");
            HttpResponse::InternalServerError().json(String::from("Failed to get records"))
        }
    }
}

#[get("{todo_id}")]
async fn get_single(todo_id: web::Path<String>) -> HttpResponse {
    match get_single_record(todo_id.to_string()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            println!("{e}");
            HttpResponse::InternalServerError()
                .json(format!("Failed to get record with ID: {}", todo_id))
        }
    }
}

#[post("")]
async fn create(todo: web::Json<TodoItem>) -> HttpResponse {
    match create_record(&todo.0).await {
        Ok(id) => HttpResponse::Ok().json(Todo {
            id: id.to_string(),
            title: todo.title.clone(),
            description: todo.description.clone(),
            due_date: todo.due_date.clone(),
        }),
        Err(e) => {
            println!("{e}");
            HttpResponse::InternalServerError().json(String::from("Failed to create record"))
        }
    }
}

#[put("{todo_id}")]
async fn update(todo_id: web::Path<String>, todo: web::Json<TodoItem>) -> HttpResponse {
    let todo: Todo = Todo {
        id: todo_id.to_string(),
        title: todo.0.title,
        description: todo.0.description,
        due_date: todo.0.due_date,
    };

    match update_record(&todo).await {
        Ok(_) => HttpResponse::Ok().json(todo),
        Err(e) => {
            println!("{e}");
            HttpResponse::InternalServerError()
                .json(format!("Failed to update record with ID: {}", todo_id))
        }
    }
}

#[delete("{todo_id}")]
async fn delete(todo_id: web::Path<String>) -> HttpResponse {
    match delete_record(todo_id.to_string()).await {
        Ok(_) => HttpResponse::Ok().body(String::from("Success")),
        Err(e) => {
            println!("{e}");
            HttpResponse::InternalServerError()
                .json(format!("Failed to delete record with ID: {}", todo_id))
        }
    }
}
