use diesel::{ prelude::*, RunQueryDsl };
use serde::{ Deserialize, Serialize, Deserializer, de::Error as DeserializeError };

use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::db::models::Notification;
use crate::db::schema::notifications;




#[derive(Deserialize, Serialize)]
pub struct NotifyGet {
    #[serde(deserialize_with = "NotifyGet::deserialize_into_num")] pub user_id: i32,
    #[serde(deserialize_with = "NotifyGet::deserialize_into_num")] pub account_id: i32
    // pub spent: Option<f32>,
    // pub was_succeed: Option<bool>
}

impl NotifyGet {
    fn deserialize_into_num<'de, D>(deserializer: D) -> Result<i32, D::Error>
        where D: Deserializer<'de>
    {
        Deserialize::deserialize( deserializer ).and_then(|v: String| {
            v.parse::<i32>().map_err( serde::de::Error::custom )
        })
    }

    /** Get order by Id **/
    pub fn get_by_id(db_conn: &PooledConn, id: i32) -> Result<Notification, AppError> {
        let result = notifications::table.find( id ).first::<Notification>( db_conn ) ?;
        Ok( result )
    }

    /** Get orders by query filter **/
    pub fn get_by_filter(connection: &PooledConn, query_data: &NotifyGet) -> Result<Vec<Notification>, AppError> {
        let mut query_complex = notifications::table
            .filter( notifications::user_id.eq( query_data.user_id ) )
            .filter( notifications::account_id.eq( query_data.account_id ) )
            .into_boxed();

        // if let Some( t ) = query_data.spent {
        //     query_complex = query_complex.filter( notifications::spent.eq( t ) );
        // }
        // if let Some( t ) = query_data.was_succeed {
        //     query_complex = query_complex.filter( notifications::was_succeed.eq( t ) );
        // }

        let results = query_complex.load::<Notification>( connection );
        results.or_else( |_| Ok( vec![] ) )
    }
}
