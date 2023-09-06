use crate::database::establish_connection;
use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;

use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

// pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
//     let state: Map<String, Value> = read_file("./state.json");
//     let status: TaskStatus;

//     match &state.get(&to_do_item.title) {
//         Some(item) => status = TaskStatus::from_string(item.as_str().unwrap().to_string()),
//         None => {
//             return HttpResponse::NotFound().json(format!("{} not in state", to_do_item.title));
//         }
//     }

//     let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
//     process_input(existing_item, "delete".to_owned(), &state);
//     return HttpResponse::Ok().json(ToDoItems::get_state());
// }

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let connection = establish_connection();

    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::delete(results).execute(&connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
