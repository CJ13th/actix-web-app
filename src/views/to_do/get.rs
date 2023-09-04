use crate::state::read_file;
use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::{enums::TaskStatus, to_do_factory, ItemTypes};

pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}
