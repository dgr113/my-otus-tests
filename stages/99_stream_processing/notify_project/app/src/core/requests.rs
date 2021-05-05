use serde::{ Serialize, Deserialize };

use crate::db::ops::create::NotifyCreate;




#[derive(Clone, Serialize, Deserialize)]
pub struct CreateNotifyData {
    pub notify: NotifyCreate
}
