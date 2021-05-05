use serde::{ Serialize, Deserialize };

use crate::kfk::event::KfkEventType;




#[derive(Clone, Serialize, Deserialize)]
pub struct OrderNotifyMsg {
    pub msg_type: KfkEventType,
    pub notify: OrderNotify
}


#[derive(Clone, Deserialize, Serialize)]
pub struct OrderNotify {
    pub customer_id: i32,
    pub account_id: i32,
    pub order_id: i32,
    pub order_amount: f32,
    pub was_succeed: bool
}
