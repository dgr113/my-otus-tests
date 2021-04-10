use serde::{ Deserialize, Serialize };
use diesel::{ RunQueryDsl, Insertable };

use crate::db::schema::users;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::CreateUserData;




#[derive(Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UserCreate {
    pub name: String
}

impl UserCreate {
    pub fn create_order(connection: PooledConn, user_data: &mut CreateUserData) -> Result<i32, AppError> {
        let user_id = diesel::insert_into( users::table ).values(  &user_data.user )
            .returning( users::id )
            .get_result( &connection ) ?;
        Ok( user_id )
    }
}
