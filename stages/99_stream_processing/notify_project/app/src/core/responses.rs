use serde::{ Serialize, Deserialize };




#[derive(Serialize, Deserialize)]
pub struct ResponseHealth {
    pub status: String
}



#[derive(Serialize, Deserialize)]
pub struct ResponseNotifyCreate {
    pub id: i32
}
