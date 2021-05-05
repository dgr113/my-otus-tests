use actix_web::web::{ Data, Query, Path, Json };

use crate::app_types::ConnPool;
use crate::core::errors::AppError;
use crate::db::models::Notification;
use crate::db::ops::read::NotifyGet;
use crate::db::ops::create::NotifyCreate;
use crate::core::requests::CreateNotifyData;
use crate::core::responses::ResponseNotifyCreate;




/** Create new user billing `Account` */
pub async fn create(conn_pool: Data<ConnPool>, mut payload: Json<CreateNotifyData>) -> Result<Json<ResponseNotifyCreate>, AppError> {
    let db_conn = conn_pool.get() ?;
    NotifyCreate::create_notify(&db_conn, &mut payload)
        .map( |id| Json( ResponseNotifyCreate { id } ) )
}



/** Get `Account` by Id */
pub async fn get_by_id(conn_pool: Data<ConnPool>, id: Path<(i32,)>) -> Result<Json<Notification>, AppError> {
    let db_conn = conn_pool.get() ?;
    let id = id.into_inner().0;
    NotifyGet::get_by_id(&db_conn, id)
        .map( Json )
}



/** Get `Notify` by filter */
pub async fn get_by_filter(conn_pool: Data<ConnPool>, query: Query<NotifyGet>) -> Result<Json<Vec<Notification>>, AppError> {
    let db_conn = conn_pool.get() ?;
    NotifyGet::get_by_filter(&db_conn, &query)
        .map( Json )
}
