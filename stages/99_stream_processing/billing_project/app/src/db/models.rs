use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, Identifiable };

use crate::db::schema::accounts;




#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub available: Option<f32>,
    pub spent: Option<f32>,
    pub create_date: NaiveDateTime,
    pub update_date: NaiveDateTime
}
