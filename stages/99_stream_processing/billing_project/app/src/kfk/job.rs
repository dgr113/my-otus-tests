use std::thread;
use std::fmt::Write;
use serde::Serialize;
use std::time::Duration;
use kafka::consumer::Consumer;
use kafka::producer::{ Producer, Record };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::kfk::event::KfkEventType;
use crate::kfk::request::OrderCreatedMsg;
use crate::db::ops::update::AccountUpdate;
use crate::core::requests::UpdateAccountData;
use crate::kfk::response::{ OrderPaidMsg, OrderPaid };




pub struct KfkJob<'a> {
    pub kfk_consumer: &'a mut Consumer,
    pub kfk_producer: &'a mut Producer,
    pub producer_topic: &'a str,
    pub db_conn: &'a ConnPool
}

impl<'a> KfkJob<'a> {
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
    pub fn start(self, interval_sec: u64) {
        let db_conn = self.db_conn.get().unwrap();
        loop {
            for ms in self.kfk_consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    // println!("BILLING SERVICE GETTING KAFKA MESSAGE: {:?}", &m);
                    let raw_json_str = String::from_utf8_lossy( m.value ).to_string();
                    match serde_json::from_str::<OrderCreatedMsg>( &raw_json_str ) {
                            Ok( order_msg ) => {
                                    match order_msg.msg_type {
                                        KfkEventType::ORDER_CREATED => {
                                            let update_data = UpdateAccountData {
                                                remove_money: Some( order_msg.order.order_amount ),
                                                add_money: None
                                            };

                                            let msg_type = match AccountUpdate::update(&db_conn, order_msg.order.account_id, &update_data) {
                                                Ok( _ ) => Ok( KfkEventType::ORDER_PAID_SUCCESS ),
                                                Err( err ) => {
                                                    println!("PAYMENT FAILED: {:}", &err.to_string());
                                                    match err {
                                                        AppError::NotEnoughMoney => Ok( KfkEventType::ORDER_PAID_FAILED ),
                                                        _ => Err( err )
                                                    }
                                                }
                                            };

                                            match msg_type {
                                                Ok( t ) => {
                                                    let kfk_msg_data = OrderPaidMsg {
                                                        msg_type: t,
                                                        order: OrderPaid {
                                                            customer_id: order_msg.order.customer_id,
                                                            account_id: order_msg.order.account_id,
                                                            order_id: order_msg.order.order_id,
                                                            order_amount: order_msg.order.order_amount
                                                        }
                                                    };
                                                    Self::send_msg(self.kfk_producer, self.producer_topic, &kfk_msg_data).expect( "Error with sending Kafka message!" );
                                                },
                                                Err( err ) => println!("ERROR UPDATE BILLING ACCOUNT: {}", &err.to_string())
                                            }
                                        },
                                        _ => println!( "UNKNOWN EVENT WAS GETTING" )
                                    }
                            },
                            Err( err ) => println!("Error with deserialize received Kafka message: {}", err.to_string())
                    }
                }
                self.kfk_consumer.consume_messageset( ms );
            }

            self.kfk_consumer.commit_consumed().unwrap();
            thread::sleep( Duration::from_secs( interval_sec ) );
        }
    }
}
