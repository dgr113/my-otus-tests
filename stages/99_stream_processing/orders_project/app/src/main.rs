extern crate openssl;
#[macro_use] extern crate maplit;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
#[macro_use] #[allow(unused_imports)] extern crate log;

use std::time::Duration;
use std::{ env, thread };
use std::sync::{ Mutex, Arc };

use dotenv::dotenv;
use actix_web::HttpServer;
use kafka::producer::{ Producer, RequiredAcks };

mod db;
mod kfk;
mod core;
mod views;
mod settings;
mod app_types;
mod app_factory;

use crate::core::errors::AppError;
use crate::db::conn::get_pg_conn_pool;
use crate::settings::{ KfkConfig, DbConfig };
use crate::app_factory::{ AppData, AppFactory };




#[actix_web::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let serv_addr = env::var( "SERVICE_ADDR" ).unwrap_or( String::from( "0.0.0.0:8888" ) );
    let kafka_config = KfkConfig::new() ?;
    let db_config = DbConfig::new() ?;
    let db_conn_str = format!("{DB_DRIVER}://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{DB_NAME}",
        DB_DRIVER = db_config.driver,
        DB_HOST = db_config.host,
        DB_PORT = db_config.port,
        DB_NAME = db_config.name,
        DB_USER = db_config.user,
        DB_PASSWORD = db_config.password
    );

    let mut kfk_producer = Producer::from_hosts( vec![ kafka_config.addr.clone() ] )
        .with_ack_timeout( Duration::from_secs( 1 ) )
        .with_required_acks( RequiredAcks::One )
        .create().expect( "Error with create Kafka producer !" );


    let db_pool = get_pg_conn_pool( &db_conn_str ) ?;
    let kfk_producer_rc = Arc::new( Mutex::new( kfk_producer ) );

    let srv = HttpServer::new(move || {
        AppFactory::create_app()
            .data( db_pool.clone() )
            .data( kafka_config.clone() )
            .data( kfk_producer_rc.clone() )
    });

    srv.bind( serv_addr ) ?
        .run()
        .await ?;

    Ok(())
}
