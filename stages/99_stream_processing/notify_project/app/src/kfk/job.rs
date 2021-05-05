use std::thread;
use std::fmt::Write;
use serde::Serialize;
use std::time::Duration;
use kafka::consumer::Consumer;
use chrono::{ NaiveDateTime, Utc };
use kafka::producer::{ Producer, Record };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::kfk::event::KfkEventType;
use crate::kfk::request::OrderPaidMsg;
use crate::db::ops::create::NotifyCreate;
use crate::core::requests::CreateNotifyData;
use crate::kfk::response::{ OrderNotifyMsg, OrderNotify };




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
                    // println!("NOTIFY SERVICE GETTING KAFKA MESSAGE: {:?}", &m);
                    let raw_json_str = String::from_utf8_lossy( m.value ).to_string();
                    match serde_json::from_str::<OrderPaidMsg>( &raw_json_str ) {
                            Ok( order_msg ) => {
                                let paid_was_succeed = match order_msg.msg_type {
                                    KfkEventType::ORDER_PAID_SUCCESS => Some( true ),
                                    KfkEventType::ORDER_PAID_FAILED => Some( false ),
                                    _ => None
                                };

                                match paid_was_succeed {
                                    Some( t ) => {
                                        let billing_account_id = order_msg.order.account_id;
                                        let order_amount = order_msg.order.order_amount;
                                        let user_id = order_msg.order.customer_id;
                                        let order_id = order_msg.order.order_id;
                                        println!("ORDER AMOUNT: {:}", &order_amount);

                                        let db_data = CreateNotifyData {
                                            notify: NotifyCreate {
                                                user_id,
                                                order_amount,
                                                account_id: billing_account_id,
                                                order_id: order_msg.order.order_id,
                                                was_succeed: t,
                                                date: Utc::now().naive_utc()
                                            }
                                        };

                                        match NotifyCreate::create_notify(&db_conn, &db_data) {
                                            Ok( _ ) => {
                                                let kfk_msg_data = OrderNotifyMsg {
                                                    msg_type: KfkEventType::ORDER_NOTIFY_SUCCESS,
                                                    notify: OrderNotify {
                                                        order_id,
                                                        order_amount,
                                                        customer_id: user_id,
                                                        account_id: billing_account_id,
                                                        was_succeed: t
                                                    }
                                                };
                                                Self::send_msg(self.kfk_producer, self.producer_topic, &kfk_msg_data).expect( "Error with sending Kafka message!" );
                                                println!( "NOTIFY SUCCESS :)" );
                                            }
                                            Err(_) => println!( "NOTIFY FAILED :(" )
                                        };
                                    },
                                    None => println!( "UNKNOWN EVENT WAS GETTING" )
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
