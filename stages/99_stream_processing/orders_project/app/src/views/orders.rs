use std::fmt::Write;
use std::time::Duration;
use std::sync::{ Arc, Mutex };

use serde::Serialize;
use serde_json::Value as SerdeValue;
use kafka::producer::{ Producer, Record, RequiredAcks };
use actix_web::web::{ Data, Query, Path, Json, HttpRequest };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::db::ops::read::OrderGet;
use crate::db::ops::update::OrderUpdate;

use crate::kfk::job::Kfk;
use crate::settings::KfkConfig;
use crate::kfk::event::KfkEventType;
use crate::db::ops::create::OrderCreate;
use crate::db::ops::delete::OrderDelete;
use crate::kfk::response::{ OrderCreatedMsg, OrderCreated };
use crate::core::responses::{ ResponseOrderGet, ResponseOrderCreate };
use crate::core::requests::{ CreateOrderData, UpdateOrderData, ReqHeader };




/** Create new order with related products */
pub async fn create(conn_pool: Data<ConnPool>, kfk_config: Data<KfkConfig>, mut kfk_data: Data<Arc<Mutex<Producer>>>, request: HttpRequest, mut payload: Json<CreateOrderData>)
    -> Result<Json<ResponseOrderCreate>, AppError>
{
    let hdr = ReqHeader::parse(&request) ?;
    let db_conn = conn_pool.get() ?;

    OrderCreate::create_order(db_conn, &mut payload)
        .and_then( |order_id| {
            let order_amount = payload.order_products.iter().map( |p| p.price * p.quantity as f32 ).sum();
            println!("ORDER AMOUNT: {:}", &order_amount);

            let kfk_msg_data = OrderCreatedMsg {
                msg_type: KfkEventType::ORDER_CREATED,
                order: OrderCreated {
                    customer_id: payload.order.customer_id,
                    account_id: hdr.account_id,
                    order_amount,
                    order_id
                }
            };

            let mut kfk_producer = kfk_data.lock()
                .map_err( |err| AppError::ThreadLockError( err.to_string() ) ) ?;

            Kfk::send_msg(&mut *kfk_producer, &kfk_config.producer_topic, &kfk_msg_data) ?;
            Ok( Json( ResponseOrderCreate { order_id } ) )
        })
}



/** Get order and related products by its Id */
pub async fn get_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<Vec<ResponseOrderGet>>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    OrderGet::get_by_id(db_conn, id).map( Json )
}



/** Get one order and related products by filter */
pub async fn get_by_filter(conn_pool: Data<ConnPool>, query: Query<OrderGet>) -> Result<Json<Vec<ResponseOrderGet>>, AppError> {
    let db_conn = conn_pool.get() ?;
    OrderGet::get_by_filter(db_conn, &query).map( Json )
}



/** Update order by its Id */
pub async fn update_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>, mut payload: Json<UpdateOrderData>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    OrderUpdate::update(db_conn, id, &mut payload.0).map( Json )
}
