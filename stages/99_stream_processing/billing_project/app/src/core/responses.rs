use serde::{ Serialize, Deserialize };

use crate::db::models::Account;




#[derive(Serialize, Deserialize)]
pub struct ResponseHealth {
    pub status: String
}



#[derive(Serialize, Deserialize)]
pub struct ResponseAccountGet {
    pub account: Account
}



#[derive(Serialize, Deserialize)]
pub struct ResponseAccountCreate {
    pub account_id: i32
}
