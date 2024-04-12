use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{todoitem, todolist};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = todoitem)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todoitem)]
pub struct NewTodoItem {
    pub list_id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = todolist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todolist)]
pub struct NewTodoList {
    pub title: String,
}
