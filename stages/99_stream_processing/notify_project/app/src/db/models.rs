use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, Identifiable };

use crate::db::schema::notifications;




#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "notifications"]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub order_id: i32,
    pub account_id: i32,
    pub order_amount: f32,
    pub was_succeed: bool,
    pub date: NaiveDateTime
}
