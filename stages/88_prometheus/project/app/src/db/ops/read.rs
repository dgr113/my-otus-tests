use serde::{ Deserialize, Serialize };
use diesel::{ prelude::*, RunQueryDsl, result::Error as DbError };

use crate::db::models::User;
use crate::db::schema::users;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;




#[derive(Deserialize, Serialize)]
pub struct UserGet {
    pub name: Option<String>
}

impl UserGet {
    /** Get order by Id **/
    pub fn get_by_id(connection: PooledConn, user_id: i32) -> Result<Vec<User>, AppError> {
        users::table.find( user_id ).load::<User>( &connection ).map_err( AppError::from )
    }

    /** Get orders by query filter **/
    pub fn get_by_filter(connection: PooledConn, query_data: &UserGet) -> Result<Vec<User>, AppError> {
        let mut query_complex = users::table.into_boxed();

        if let Some(t) = &query_data.name {
            query_complex = query_complex.filter( users::name.eq( t ) );
        }

        let results = connection.transaction::<Vec<User>, DbError, _>(|| {
            query_complex.load::<User>( &connection )
        });

        results.or_else( |_| Ok( vec![] ) )
    }
}
