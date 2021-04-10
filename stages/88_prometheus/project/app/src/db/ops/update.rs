use serde::{ Deserialize, Serialize };
use diesel::{
    prelude::*,
    RunQueryDsl, Insertable,
    result::Error as DbError
};

use crate::db::schema::users;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::UpdateUserData;




#[derive(Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UserUpdate {
    pub name: String
}

impl UserUpdate {
    /** Update order **/
    pub fn update(connection: PooledConn, user_id: i32, user_data: &mut UpdateUserData) -> Result<(), AppError> {
        connection.transaction::<(), DbError, _>(|| {
            let update_result = diesel::update( users::table.find( user_id ) )
                .set( &user_data.user )
                .execute( &connection );

            match update_result == Ok(0) {
                true => Err( DbError::NotFound ),  // No any records found and updated
                false => Ok(())  // Successfully updated
            }
        }) ?;
        Ok(())
    }
}
