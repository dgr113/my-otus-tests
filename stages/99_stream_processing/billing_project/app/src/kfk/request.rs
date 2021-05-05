use serde::{ Serialize, Deserialize };

use crate::kfk::event::KfkEventType;




#[derive(Clone, Serialize, Deserialize)]
pub struct OrderCreatedMsg {
    pub msg_type: KfkEventType,
    pub order: OrderCreated
}


#[derive(Clone, Deserialize, Serialize)]
pub struct OrderCreated {
    pub customer_id: i32,
    pub account_id: i32,
    pub order_id: i32,
    pub order_amount: f32
}
