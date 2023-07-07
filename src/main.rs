mod dynamo;
mod todo;
use crate::todo::routes::configure_todo_scope;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_todo_scope))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
