use serde::{ Serialize, Deserialize };

use crate::db::ops::update::OrderUpdate;
use crate::db::ops::create::{ OrderProductCreate, OrderCreate };




#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderData {
    pub idempotency_key: String,
    pub order: OrderCreate,
    pub order_products: Vec<OrderProductCreate>
}



#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateOrderData {
    pub order: OrderUpdate
}
