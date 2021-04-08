use chrono::{NaiveDateTime, Utc};
use serde::{ Deserialize, Serialize };
use diesel::{ prelude::*, RunQueryDsl, Insertable, result::Error as DbError };

use crate::app_types::PooledConn;
use crate::core::errors::AppError;
use crate::core::requests::CreateOrderData;
use crate::db::schema::{ orders, order_products, order_unique_keys };




#[derive(Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name="order_products"]
pub struct OrderProductCreate {
    pub price: i32,
    pub quantity: i32,
    pub product_id: i32,
    pub product_name: String,
    #[serde(skip)] pub order_id: Option<i32>
}



#[derive(Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name="orders"]
pub struct OrderCreate {
    pub customer_id: i32,
    #[serde(skip)] pub status_code: Option<i32>,
    #[serde(skip)] pub create_date: Option<NaiveDateTime>,
    #[serde(skip)] pub update_date: Option<NaiveDateTime>
}

impl OrderCreate {
    /** Insert a record and get its ID (SQLite concept) **/
    fn _create_order(connection: &PooledConn, order: &mut OrderCreate, idempotency_key: &str) -> Result<i32, DbError> {
        let order_id = diesel::insert_into( orders::table ).values( &*order )
            .returning( orders::id )
            .get_result( connection ) ?;

        diesel::insert_into( order_unique_keys::table )
            .values((
                order_unique_keys::unique_key.eq( idempotency_key ),
                order_unique_keys::order_id.eq( order_id )
            ))
            .execute( connection ) ?;

        Ok( order_id )
    }

    /** Iteratively write each row into dependent database (SQLite concept) **/
    fn _create_related_products(connection: &PooledConn, order_id: i32, items: &Vec<OrderProductCreate>) -> Result<(), DbError> {
        for order_item in items {
            let mut rec = OrderProductCreate::from( order_item.clone() );
            rec.order_id = Some( order_id );
            diesel::insert_into( order_products::table ).values( rec ).execute( connection ) ?;
        };
        Ok(())
    }

    /** Insert records with relations from new order data **/
    pub fn create_order(connection: PooledConn, order_data: &mut CreateOrderData) -> Result<i32, AppError> {
        let status_code = 0;  // Status for new Order
        order_data.order.status_code = Some( status_code );
        order_data.order.create_date = Some( Utc::now().naive_utc() );
        order_data.order.update_date = Some( Utc::now().naive_utc() );

        let order_id = connection.transaction::<i32, DbError, _>(|| {
            let order_id = Self::_create_order(&connection, &mut order_data.order, &order_data.idempotency_key) ?;
            Self::_create_related_products(&connection, order_id, &order_data.order_products) ?;
            Ok( order_id )
        }) ?;
        Ok( order_id )
    }
}
