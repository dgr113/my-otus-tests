use actix_web::web::{ Data, Query, Path, Json };

use crate::app_types::ConnPool;
use crate::db::models::Account;
use crate::core::errors::AppError;
use crate::db::ops::read::AccountGet;
use crate::db::ops::update::AccountUpdate;
use crate::db::ops::create::AccountCreate;
use crate::db::ops::delete::AccountDelete;
use crate::core::requests::{ CreateAccountData, UpdateAccountData };
use crate::core::responses::{ ResponseAccountGet, ResponseAccountCreate };




/** Create new user billing `Account` */
pub async fn create(conn_pool: Data<ConnPool>, mut payload: Json<CreateAccountData>) -> Result<Json<ResponseAccountCreate>, AppError> {
    let db_conn = conn_pool.get() ?;
    AccountCreate::create_account(&db_conn, &mut payload)
        .map( |account_id| Json( ResponseAccountCreate { account_id } ) )
}



/** Get `Account` by Id */
pub async fn get_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<Account>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    AccountGet::get_by_id(&db_conn, id).map( Json )
}



/** Update `Account` by its Id */
pub async fn update_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>, payload: Json<UpdateAccountData>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    AccountUpdate::update(&db_conn, id, &payload.0)
        .map( Json )
}



/** Delete `Account` by its Id */
pub async fn delete_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    AccountDelete::delete(&db_conn, id)
        .map( Json )
}
