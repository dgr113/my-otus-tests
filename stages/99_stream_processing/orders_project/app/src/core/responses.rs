use serde::{ Serialize, Deserialize };

use crate::db::models::{ OrderProduct, Order };




#[derive(Serialize, Deserialize)]
pub struct ResponseHealth {
    pub status: String
}



#[derive(Serialize, Deserialize)]
pub struct ResponseOrderGet {
    pub order: Order,
    pub order_products: Vec<OrderProduct>
}



#[derive(Serialize, Deserialize)]
pub struct ResponseOrderCreate {
    pub order_id: i32
}
