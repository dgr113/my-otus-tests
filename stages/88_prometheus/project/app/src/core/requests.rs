use serde::{ Serialize, Deserialize };

use crate::db::ops::update::UserUpdate;
use crate::db::ops::create::UserCreate;




#[derive(Clone, Serialize, Deserialize)]
pub struct CreateUserData {
    pub user: UserCreate,
}



#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateUserData {
    pub user: UserUpdate
}
