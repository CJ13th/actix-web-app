use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use serde::Serialize;

use std::vec::Vec;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};

use crate::to_do::{enums::TaskStatus, to_do_factory};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
            }
        }
        let done_count = done_array_buffer.len() as i8;
        let pending_count = pending_array_buffer.len() as i8;

        ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        // let state = read_file("./state.json");
        // let mut array_buffer = Vec::new();

        // for (key, value) in state {
        //     let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
        //     let item = to_do_factory(&key, status);
        //     array_buffer.push(item);
        // }
        // return ToDoItems::new(array_buffer);

        let connection = establish_connection();
        let mut array_buffer = Vec::new();

        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();

        for item in items {
            let status = TaskStatus::from_string(item.status);
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }
        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
