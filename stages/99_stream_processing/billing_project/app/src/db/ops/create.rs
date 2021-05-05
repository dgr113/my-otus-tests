use chrono::{ NaiveDateTime, Utc };
use serde::{ Deserialize, Serialize };
use diesel::{ prelude::*, RunQueryDsl, Insertable, result::Error as DbError };

use crate::db::schema::accounts;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::CreateAccountData;




#[derive(Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name="accounts"]
pub struct AccountCreate {
    pub user_id: i32,
    pub user_name: String,
    #[serde(skip)] pub available: Option<f32>,
    #[serde(skip)] pub spent: Option<f32>,
    #[serde(skip)] pub create_date: Option<NaiveDateTime>,
    #[serde(skip)] pub update_date: Option<NaiveDateTime>
}

impl AccountCreate {
    /** Insert a record and get its ID (SQLite concept) **/
    fn _create_account(db_conn: &PooledConn, data: &mut AccountCreate, idempotency_key: &str) -> Result<i32, DbError> {
        let _id = diesel::insert_into( accounts::table ).values( &*data )
            .returning( accounts::id )
            .get_result( db_conn ) ?;

        Ok( _id )
    }

    /** Insert records with relations from new order data **/
    pub fn create_account(db_conn: &PooledConn, data: &mut CreateAccountData) -> Result<i32, AppError> {
        data.account.create_date = Some( Utc::now().naive_utc() );
        data.account.update_date = Some( Utc::now().naive_utc() );
        data.account.available = Some( 0.0 );
        data.account.spent = Some( 0.0 );

        let _id = db_conn.transaction::<i32, DbError, _>(|| {
            let result = Self::_create_account(db_conn, &mut data.account, &data.idempotency_key) ?;
            Ok( result )
        }) ?;

        Ok( _id )
    }
}
