use chrono::{ NaiveDateTime, Utc };
use serde::{ Deserialize, Serialize };
use diesel::{
    prelude::*,
    RunQueryDsl, Insertable,
    result::Error as DbError
};

use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::UpdateOrderData;
use crate::db::schema::orders;




#[derive(Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name="orders"]
pub struct OrderUpdate {
    pub status_code: i32,
    #[serde(skip)] pub update_date: Option<NaiveDateTime>
}

impl OrderUpdate {
    /** Update order **/
    pub fn update(connection: PooledConn, order_id: i32, order_data: &mut UpdateOrderData) -> Result<(), AppError> {
        connection.transaction::<(), DbError, _>(|| {
            // order_data.order.create_date = Some( Utc::now().naive_utc() );
            order_data.order.update_date = Some( Utc::now().naive_utc() );

            let res = diesel::update( orders::table.find( order_id ) )
                .set( &order_data.order )
                .execute( &connection );

            match res == Ok(0) {
                true => Err( DbError::NotFound ),  // No any records found and updated
                false => Ok(())  // Successfully updated
            }
        }) ?;
        Ok(())
    }
}
