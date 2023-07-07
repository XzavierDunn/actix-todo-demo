use crate::todo::model::{Todo, TodoItem};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::{
    error::SdkError,
    operation::{
        delete_item::DeleteItemError, put_item::PutItemError, query::QueryError, scan::ScanError,
    },
    types::{AttributeValue, Select},
    Client,
};
use serde_dynamo::from_items;
use uuid::Uuid;

pub async fn get_records() -> Result<Vec<Todo>, SdkError<ScanError>> {
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    match client.scan().table_name("temp").send().await {
        Ok(resp) => match resp.items {
            Some(items) => {
                let todos: Vec<Todo> = from_items(items).expect("items");
                Ok(todos)
            }
            None => Ok(Vec::new()),
        },
        Err(e) => Err(e),
    }
}

pub async fn get_single_record(id: String) -> Result<Vec<Todo>, SdkError<QueryError>> {
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    match client
        .query()
        .table_name("temp")
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "id".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(id))
        .select(Select::AllAttributes)
        .send()
        .await
    {
        Ok(resp) => match resp.items {
            Some(items) => {
                let todos: Vec<Todo> = from_items(items).expect("items");
                Ok(todos)
            }
            None => Ok(Vec::new()),
        },
        Err(e) => Err(e),
    }
}

pub async fn create_record(todo: &TodoItem) -> Result<Uuid, SdkError<PutItemError>> {
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    let id = Uuid::new_v4();
    let title = AttributeValue::S(todo.title.clone());
    let description = AttributeValue::S(todo.description.clone());
    let due_date = AttributeValue::S(todo.due_date.clone());

    match client
        .put_item()
        .table_name("temp")
        .item("id", AttributeValue::S(id.to_string()))
        .item("title", title)
        .item("description", description)
        .item("due_date", due_date)
        .send()
        .await
    {
        Ok(_) => Ok(id),
        Err(e) => Err(e),
    }
}

pub async fn update_record(todo: &Todo) -> Result<(), SdkError<PutItemError>> {
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    let id = AttributeValue::S(todo.id.clone());
    let title = AttributeValue::S(todo.title.clone());
    let description = AttributeValue::S(todo.description.clone());
    let due_date = AttributeValue::S(todo.due_date.clone());

    match client
        .put_item()
        .table_name("temp")
        .item("id", id)
        .item("title", title)
        .item("description", description)
        .item("due_date", due_date)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn delete_record(id: String) -> Result<(), SdkError<DeleteItemError>> {
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    match client
        .delete_item()
        .table_name("temp")
        .key("id", AttributeValue::S(id))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
