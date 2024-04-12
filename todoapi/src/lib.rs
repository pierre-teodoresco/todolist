use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

/* MODULES */
pub mod actions;
pub mod handlers;
pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[cfg(feature = "dev")]
use dotenv;

mod envhelper {
    use std::env;

    pub fn get_db_url() -> String {
        #[cfg(feature = "dev")]
        {
            dotenv::dotenv().ok();
            println!("Env variables loaded from .env file");
        }
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }
}

pub fn get_connection_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(envhelper::get_db_url());

    Pool::builder()
        .build(manager)
        .expect("Error while trying to connect to postegreSQL")
}
