use crate::models::{NewTodoItem, NewTodoList, TodoItem, TodoList};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn insert_new_list(conn: &mut PgConnection, title: String) -> TodoList {
    use crate::schema::todolist;

    let new_list = NewTodoList { title };

    diesel::insert_into(todolist::table)
        .values(&new_list)
        .returning(TodoList::as_returning())
        .get_result(conn)
        .expect("Error saving new item")
}

pub fn insert_new_item(conn: &mut PgConnection, list_id: i32, title: String) -> TodoItem {
    use crate::schema::todoitem;

    let new_item = NewTodoItem { list_id, title };

    diesel::insert_into(todoitem::table)
        .values(&new_item)
        .returning(TodoItem::as_returning())
        .get_result(conn)
        .expect("Error saving new item")
}

pub fn fetch_all_items_from_list(conn: &mut PgConnection, list: i32) -> Vec<TodoItem> {
    use crate::schema::todoitem::dsl::*;

    todoitem
        .filter(list_id.eq(list))
        .select(TodoItem::as_select())
        .load(conn)
        .expect("Error loading items")
}

pub fn fetch_all_lists(conn: &mut PgConnection, limit: i64) -> Vec<TodoList> {
    use crate::schema::todolist::dsl::*;

    todolist
        .limit(limit)
        .select(TodoList::as_select())
        .load(conn)
        .expect("Error loading lists")
}

pub fn delete_item(conn: &mut PgConnection, item_id: i32) {
    use crate::schema::todoitem::dsl::*;

    diesel::delete(todoitem.filter(id.eq(item_id)))
        .execute(conn)
        .expect("Error deleting item");
}

pub fn update_item(
    conn: &mut PgConnection,
    item_id: i32,
    new_title: String,
    new_completed: bool,
) -> TodoItem {
    use crate::schema::todoitem::dsl::*;

    diesel::update(todoitem.find(item_id))
        .set((title.eq(new_title), completed.eq(new_completed)))
        .returning(TodoItem::as_returning())
        .get_result(conn)
        .unwrap()
}
