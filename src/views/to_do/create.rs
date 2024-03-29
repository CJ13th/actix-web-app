use crate::diesel;
use actix_web::HttpRequest;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::database::DB;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::HttpResponse;

// pub async fn create(req: HttpRequest) -> String {
//     let state: Map<String, Value> = read_file("./state.json");
//     let title = req.match_info().get("title").unwrap().to_string();
//     let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
//     process_input(item, "create".to_string(), &state);
//     return format!("{} created", title);
// }
// pub async fn create(req: HttpRequest) -> HttpResponse {
//     let state: Map<String, Value> = read_file("./state.json");
//     let title: String = req.match_info().get("title").unwrap().to_string();
//     let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
//     process_input(item, "create".to_string(), &state);
//     return HttpResponse::Ok().json(ToDoItems::get_state());
// }

pub async fn create(req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    // let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&db.connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
