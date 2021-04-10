extern crate openssl;
#[macro_use] extern crate maplit;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] #[allow(unused_imports)] extern crate log;

use std::env;
use dotenv::dotenv;
use actix_web::HttpServer;

use crate::core::errors::AppError;
use crate::app_factory::{ AppData, AppFactory };
use crate::prometheus::metrics::register_custom_metrics;
use crate::db::conn::get_pg_conn_pool;

mod db;
mod core;
mod views;
mod app_types;
mod app_factory;
mod prometheus;




#[actix_web::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    register_custom_metrics();

    let db_path = format!("{DB_DRIVER}://{DB_USERNAME}:{DB_PASSWD}@{DB_HOST}:{DB_PORT}/{DB_NAME}",
        DB_DRIVER = env::var("DB_DRIVER") ?,
        DB_HOST = env::var("DB_HOST") ?,
        DB_PORT = env::var("DB_PORT") ?,
        DB_NAME = env::var("DB_NAME") ?,
        DB_USERNAME = env::var("DB_USERNAME") ?,
        DB_PASSWD = env::var("DB_PASSWD") ?
    );

    let db_pool = get_pg_conn_pool( &db_path ) ?;
    let app_data = AppData { db_pool };

    let serv_addr = env::var( "SERVICE_ADDR" ).unwrap_or( String::from( "0.0.0.0:8888" ) );

    let serv = HttpServer::new(move || {
        AppFactory::create_app().data( app_data.clone().db_pool )
    });

    serv.bind( serv_addr ) ?
        .run()
        .await ?;

    Ok(())
}
