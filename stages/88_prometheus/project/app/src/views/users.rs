use actix_web::web::{ Data, Query, Path, Json };

use crate::db::models::User;
use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::db::ops::read::UserGet;
use crate::db::ops::create::UserCreate;
use crate::db::ops::update::UserUpdate;
use crate::db::ops::delete::UserDelete;
use crate::core::responses::ResponseUserCreate;
use crate::core::requests::{ CreateUserData, UpdateUserData };




/** Create new `User` */
pub async fn create(conn_pool: Data<ConnPool>, mut payload: Json<CreateUserData>) -> Result<Json<ResponseUserCreate>, AppError> {
    let db_conn = conn_pool.get() ?;
    UserCreate::create_order(db_conn, &mut payload)
        .map( |user_id| Json( ResponseUserCreate { user_id } ) )
}



/** Get `User` by its Id */
pub async fn get_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<Vec<User>>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    UserGet::get_by_id(db_conn, id).map( Json )
}



/** Get `Users` by filter */
pub async fn get_by_filter(conn_pool: Data<ConnPool>, query: Query<UserGet>) -> Result<Json<Vec<User>>, AppError> {
    let db_conn = conn_pool.get() ?;
    UserGet::get_by_filter(db_conn, &query).map( Json )
}



/** Update `User` by its Id */
pub async fn update_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>, mut payload: Json<UpdateUserData>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    UserUpdate::update(db_conn, id, &mut payload).map( Json )
}



/** Delete `User` by its Id */
pub async fn delete_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<()>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    UserDelete::delete(db_conn, id).map( Json )
}
