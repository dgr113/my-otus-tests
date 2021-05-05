use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize };
use diesel::{ prelude::*, RunQueryDsl, result::Error as DbError };

use crate::core::errors::AppError;
use crate::app_types::PooledConn;
use crate::db::schema::orders;
use crate::core::responses::ResponseOrderGet;
use crate::db::models::{ Order, OrderProduct };




#[derive(Deserialize, Serialize)]
pub struct OrderGet {
    pub date_left: Option<NaiveDateTime>,
    pub date_right: Option<NaiveDateTime>,
    pub status_code: Option<i32>
}

impl OrderGet {
    /** Get orders items **/
    fn _get_related_products(connection: &PooledConn, orders: Vec<Order>) -> Result<Vec<ResponseOrderGet>, DbError> {
        let results = orders.into_iter().map(|order| {
            let order_products = OrderProduct::belonging_to( &order ).load( connection ).expect("Error with loading order related products!");
            ResponseOrderGet { order, order_products }
        }).collect();
        Ok( results )
    }

    /** Get order by Id **/
    pub fn get_by_id(connection: PooledConn, order_id: i32) -> Result<Vec<ResponseOrderGet>, AppError> {
        let orders_data = orders::table.find( order_id ).load::<Order>( &connection ) ?;
        Self::_get_related_products(&connection, orders_data).map_err( AppError::from )
    }

    /** Get orders by query filter **/
    pub fn get_by_filter(connection: PooledConn, query_data: &OrderGet) -> Result<Vec<ResponseOrderGet>, AppError> {
        let mut query_complex = orders::table.into_boxed();

        if let Some(t) = &query_data.date_left {
            query_complex = query_complex.filter( orders::create_date.ge( t ) );
        }
        if let Some(t) = &query_data.date_right {
            query_complex = query_complex.filter( orders::update_date.le( t ) );
        }
        if let Some(t) = &query_data.status_code {
            query_complex = query_complex.filter( orders::status_code.eq( t ) );
        }

        let results = connection.transaction::<Vec<ResponseOrderGet>, DbError, _>(|| {
            let orders = query_complex.load::<Order>( &connection ) ?;
            Self::_get_related_products(&connection, orders)
        });

        results.or_else( |_| Ok( vec![] ) )
    }
}
