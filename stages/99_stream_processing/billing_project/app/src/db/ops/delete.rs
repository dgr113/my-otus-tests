use diesel::{ prelude::*, RunQueryDsl, result::Error as DbError };

use crate::db::schema::accounts;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;




pub struct AccountDelete;

impl AccountDelete {
    /** Update order **/
    pub fn delete(db_conn: &PooledConn, id: i32) -> Result<(), AppError> {
        db_conn.transaction::<(), DbError, _>(|| {
            diesel::delete( accounts::table.find( id ) ).execute( db_conn ) ?;
            Ok(())
        }) ?;
        Ok(())
    }
}
