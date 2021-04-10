use serde::{ Serialize, Deserialize };




#[derive(Serialize, Deserialize)]
pub struct ResponseHealth {
    pub status: String
}



#[derive(Serialize, Deserialize)]
pub struct ResponseUserCreate {
    pub user_id: i32
}
