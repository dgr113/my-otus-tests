use std::thread;
use std::fmt::Write;
use serde::Serialize;
use std::time::Duration;
use kafka::consumer::Consumer;
use kafka::producer::{ Producer, Record };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::kfk::event::KfkEventType;




pub struct Kfk<'a> {
    pub kfk_producer: &'a mut Producer,
    pub producer_topic: &'a str,
    pub db_conn: &'a ConnPool
}

impl<'a> Kfk<'a> {
    /** Send Kafka message into producer topic */
    pub fn send_msg<T>(kfk_producer: &mut Producer, kfk_producer_topic: &str, kfk_msg_data: &T) -> Result<(), AppError>
        where T: Sized + Serialize
    {
        let response_msg = serde_json::to_string( &kfk_msg_data ) ?;

        let mut buf = String::with_capacity( 2 );
        write!(&mut buf, "{}", response_msg);  // Some computation of the message data to be sent
        let kfk_msg = Record::from_value(kfk_producer_topic, buf.as_bytes());
        kfk_producer.send( &kfk_msg ) ?;
        buf.clear();

        Ok(())
    }

    /** Start Kafka consume process */
    #[allow(dead_code)]
    pub fn start_consume(self, _interval_sec: u64) {  }
}
