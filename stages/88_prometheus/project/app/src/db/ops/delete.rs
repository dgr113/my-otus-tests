use diesel::{ prelude::*, RunQueryDsl, result::Error as DbError };

use crate::db::schema::users;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;




pub struct UserDelete;

impl UserDelete {
    /** Update order **/
    pub fn delete(connection: PooledConn, user_id: i32) -> Result<(), AppError> {
        connection.transaction::<(), DbError, _>(|| {
            diesel::delete( users::table.find( user_id ) ).execute( &connection ) ?;
            Ok(())
        }) ?;
        Ok(())
    }
}
