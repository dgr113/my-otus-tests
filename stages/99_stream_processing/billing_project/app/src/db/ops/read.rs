use diesel::{ prelude::*, RunQueryDsl };

use crate::db::models::Account;
use crate::db::schema::accounts;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;




pub struct AccountGet;

impl AccountGet {
    /** Get order by Id **/
    pub fn get_by_id(db_conn: &PooledConn, id: i32) -> Result<Account, AppError> {
        let result = accounts::table.find( id ).first::<Account>( db_conn ) ?;
        Ok( result )
    }
}
