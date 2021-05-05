use chrono::{ NaiveDateTime, Utc };
use serde::{ Deserialize, Serialize };
use diesel::{ prelude::*, RunQueryDsl, Insertable, result::Error as DbError };

use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::db::schema::notifications;
use crate::core::requests::CreateNotifyData;




#[derive(Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name="notifications"]
pub struct NotifyCreate {
    pub user_id: i32,
    pub order_id: i32,
    pub account_id: i32,
    pub order_amount: f32,
    pub was_succeed: bool,
    pub date: NaiveDateTime
}

impl NotifyCreate {
    /** Insert a record and get its ID (SQLite concept) **/
    fn _create_notify(db_conn: &PooledConn, data: &NotifyCreate) -> Result<i32, DbError> {
        let _id = diesel::insert_into( notifications::table ).values( &*data )
            .returning( notifications::id )
            .get_result( db_conn ) ?;
        Ok( _id )
    }

    /** Insert records with relations from new order data **/
    pub fn create_notify(db_conn: &PooledConn, data: &CreateNotifyData) -> Result<i32, AppError> {
        let _id = db_conn.transaction::<i32, DbError, _>(|| {
            let result = Self::_create_notify(db_conn, &data.notify) ?;
            Ok( result )
        }) ?;
        Ok( _id )
    }
}
