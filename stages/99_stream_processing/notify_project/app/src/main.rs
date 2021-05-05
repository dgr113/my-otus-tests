extern crate openssl;
#[macro_use] extern crate maplit;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
#[macro_use] #[allow(unused_imports)] extern crate log;

use std::sync::Mutex;
use std::time::Duration;
use std::{ env, thread };

use dotenv::dotenv;
use actix_web::HttpServer;
use kafka::producer::{ Producer, RequiredAcks };
use kafka::consumer::{ GroupOffsetStorage, Consumer, FetchOffset };

mod db;
mod kfk;
mod core;
mod views;
mod settings;
mod app_types;
mod app_factory;

use crate::kfk::job::KfkJob;
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

    let mut kfk_consumer = Consumer::from_hosts( vec![ kafka_config.addr.clone() ] )
        .with_topic( kafka_config.consumer_topic.clone() )
        // .with_topic_partitions(kafka_config.consumer_topic.clone(), &[0, 1])
        .with_fallback_offset( FetchOffset::Earliest )
        .with_group( kafka_config.group.clone() )
        .with_offset_storage( GroupOffsetStorage::Kafka )
        .create().expect( "Error with create Kafka consumer!" );

    let mut kfk_producer = Producer::from_hosts( vec![ kafka_config.addr.clone() ] )
        .with_ack_timeout( Duration::from_secs( 1 ) )
        .with_required_acks( RequiredAcks::One )
        .create().expect( "Error with create Kafka producer !" );


    let db_pool = get_pg_conn_pool( &db_conn_str ) ?;

    //// RUN KAFKA CONSUMER
    thread::spawn({
        let db_pool = db_pool.clone();
        move || {
            let consumer_job = KfkJob {
                kfk_consumer: &mut kfk_consumer,
                kfk_producer: &mut kfk_producer,
                producer_topic: &kafka_config.producer_topic,
                db_conn: &db_pool
            };
            consumer_job.start( 1 );
        }
    });
    ////

    let srv = HttpServer::new(move || {
        AppFactory::create_app()
            .data( db_pool.clone() )
    });

    srv.bind( serv_addr ) ?
        .run()
        .await ?;

    Ok(())
}
