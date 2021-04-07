use serde::{ Serialize, Deserialize };

use crate::db::models::{ OrderProduct, Order };




#[derive(Serialize, Deserialize)]
pub struct ResponseHealth {
    pub status: String
}



#[derive(Serialize, Deserialize)]
pub struct ResponseOrder {
    pub order: Order,
    pub order_products: Vec<OrderProduct>
}
