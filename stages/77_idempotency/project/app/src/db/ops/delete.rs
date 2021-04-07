use diesel::{ prelude::*, RunQueryDsl, result::Error as DbError };

use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::db::schema::orders;




pub struct OrderDelete;

impl OrderDelete {
    /** Update order **/
    pub fn delete(connection: PooledConn, order_id: i32) -> Result<(), AppError> {
        connection.transaction::<(), DbError, _>(|| {
            diesel::delete( orders::table.find( order_id ) ).execute( &connection ) ?;
            Ok(())
        }) ?;
        Ok(())
    }
}
