use actix_web::{web, App, HttpServer};
use std::env;
use todoapi::{get_connection_pool, handlers};

#[cfg(feature = "dev")]
use dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "dev")]
    {
        dotenv::dotenv().ok();
        println!("Env variables loaded from .env file");
    }

    let api_addr = env::var("ADDR").unwrap_or(String::from("localhost"));
    let api_port = env::var("PORT")
        .unwrap_or(8080.to_string())
        .parse()
        .expect("PORT must be a valid integer");

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/todo")
                .route("/create-item", web::post().to(handlers::create_todo_item))
                .route("/create-list", web::post().to(handlers::create_todo_list))
                .route("/get-items/{list_id}", web::get().to(handlers::get_items))
                .route("/get-lists", web::get().to(handlers::get_lists))
                .route("/delete-item", web::delete().to(handlers::delete_item))
                .route("/update-item", web::put().to(handlers::update_item)),
        )
    })
    .bind((api_addr, api_port))?
    .run()
    .await
}
