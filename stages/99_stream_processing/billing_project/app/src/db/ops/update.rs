use chrono::{ NaiveDateTime, Utc };
use diesel::{ prelude::*, RunQueryDsl, Insertable, result::Error as DbError };

use crate::db::models::Account;
use crate::db::schema::accounts;
use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::UpdateAccountData;




#[derive(Clone, Insertable, AsChangeset)]
#[table_name="accounts"]
pub struct AccountUpdate {
    pub available: Option<f32>,
    pub spent: Option<f32>,
    pub update_date: Option<NaiveDateTime>
}

impl AccountUpdate {
    /** Update order **/
    pub fn update(db_conn: &PooledConn, id: i32, data: &UpdateAccountData) -> Result<(), AppError> {
        db_conn.transaction::<(), AppError, _>(|| {
            let acc: Account = accounts::table.find( id ).first( db_conn ) ?;
            let current_available = acc.available.unwrap_or( 0.0 );

            let add_money = data.add_money.unwrap_or( 0.0 ).abs();
            let remove_money = data.remove_money.unwrap_or( 0.0 ).abs();
            let new_diff_money = add_money - remove_money;

            if current_available + new_diff_money > 0.0 {
                let new_available = Some( current_available + new_diff_money );
                let new_spent = acc.spent.map( |t| t + remove_money );

                let result = diesel::update( accounts::table.find( id ) )
                    .set( &AccountUpdate {
                        available: new_available,
                        spent: new_spent,
                        update_date: Some( Utc::now().naive_utc() )
                    })
                    .execute( db_conn );

                match result == Ok(0) {
                    true => Err( AppError::DbError("Updated billing account error!".to_string()) ),  // No any records found and updated
                    false => Ok(())  // Successfully updated
                }
            } else {
                Err( AppError::NotEnoughMoney )
            }
        })
    }
}
