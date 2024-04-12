use crate::actions;
use crate::models::{NewTodoItem, NewTodoList};
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn create_todo_list(
    pool: web::Data<DbPool>,
    list: web::Json<NewTodoList>,
) -> actix_web::Result<impl Responder> {
    let title = list.0.title;

    let todo_list = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::insert_new_list(&mut conn, title)
    })
    .await?;

    println!("List {} has been inserted", todo_list.title);
    actix_web::Result::Ok(HttpResponse::Ok().json(todo_list))
}

pub async fn create_todo_item(
    pool: web::Data<DbPool>,
    item: web::Json<NewTodoItem>,
) -> actix_web::Result<impl Responder> {
    let list_id = item.0.list_id;
    let title = item.0.title;

    let todo_item = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::insert_new_item(&mut conn, list_id, title)
    })
    .await?;

    println!("Item {} has been inserted", todo_item.title);
    actix_web::Result::Ok(HttpResponse::Ok().json(todo_item))
}

pub async fn get_items(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let list_id = path.into_inner();

    let items = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::fetch_all_items_from_list(&mut conn, list_id)
    })
    .await?;

    println!("Items {:?} have been selected", items);
    actix_web::Result::Ok(HttpResponse::Ok().json(items))
}

#[derive(Deserialize)]
pub struct GetListsQP {
    limit: Option<i64>,
}

pub async fn get_lists(
    pool: web::Data<DbPool>,
    query: web::Query<GetListsQP>,
) -> actix_web::Result<impl Responder> {
    let limit = query.limit.unwrap_or(10);

    let lists = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::fetch_all_lists(&mut conn, limit)
    })
    .await?;

    println!(
        "The {}th firsts lists {:?} have been selected",
        limit, lists
    );
    actix_web::Result::Ok(HttpResponse::Ok().json(lists))
}

#[derive(Deserialize)]
pub struct DelItemQP {
    id: i32,
}

pub async fn delete_item(
    pool: web::Data<DbPool>,
    query: web::Query<DelItemQP>,
) -> actix_web::Result<impl Responder> {
    let id = query.id;

    web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::delete_item(&mut conn, id)
    })
    .await?;

    println!("The item {} has been deleted", id,);
    actix_web::Result::Ok(HttpResponse::Ok().json(format!("Item {} has been deleted", id)))
}

#[derive(Deserialize)]
pub struct PutItemJSON {
    id: i32,
    title: String,
    completed: bool,
}

pub async fn update_item(
    pool: web::Data<DbPool>,
    item: web::Json<PutItemJSON>,
) -> actix_web::Result<impl Responder> {
    let id = item.0.id;
    let title = item.0.title;
    let completed = item.0.completed;

    let item = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::update_item(&mut conn, id, title, completed)
    })
    .await?;

    println!("Updated item {:?}", item);
    actix_web::Result::Ok(HttpResponse::Ok().json(item))
}
