use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, Identifiable };

use crate::db::schema::{ orders, order_products, order_unique_keys };




#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub create_date: NaiveDateTime,
    pub update_date: NaiveDateTime,
    pub customer_id: i32,
    pub status_code: i32
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations)]
#[belongs_to(parent = "Order", foreign_key = "order_id")]
#[table_name = "order_products"]
pub struct OrderProduct {
    pub id: i32,
    pub order_id: i32,
    pub price: i32,
    pub quantity: i32,
    pub product_name: String,
    pub product_id: Option<i32>
}



#[derive(Debug, PartialEq, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations)]
#[belongs_to(parent = "Order", foreign_key = "order_id")]
#[table_name = "order_unique_keys"]
pub struct OrderUniqueKey {
    pub id: i32,
    pub order_id: i32,
    pub unique_key: String
}
