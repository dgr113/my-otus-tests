use serde::{ Serialize, Deserialize };

use crate::db::ops::update::AccountUpdate;
use crate::db::ops::create::AccountCreate;




#[derive(Clone, Serialize, Deserialize)]
pub struct CreateAccountData {
    pub idempotency_key: String,
    pub account: AccountCreate
}



#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateAccountData {
    pub add_money: Option<f32>,
    pub remove_money: Option<f32>
}
