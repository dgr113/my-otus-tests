use actix_web::web::{ Data, Query, Path, Json };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::db::ops::read::OrderGet;
use crate::db::ops::create::OrderCreate;
use crate::db::ops::update::OrderUpdate;
use crate::db::ops::delete::OrderDelete;
use crate::core::requests::{ CreateOrderData, UpdateOrderData };
use crate::core::responses::{ ResponseOrderGet, ResponseOrderCreate };




/** Create new order with related products */
pub async fn create(conn_pool: Data<ConnPool>, mut payload: Json<CreateOrderData>) -> Result<Json<ResponseOrderCreate>, AppError> {
    let db_conn = conn_pool.get() ?;
    OrderCreate::create_order(db_conn, &mut payload)
        .map( |order_id| Json( ResponseOrderCreate { order_id } ) )
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



/** Delete order by its Id */
pub async fn delete_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    OrderDelete::delete(db_conn, id).map( Json )
}
