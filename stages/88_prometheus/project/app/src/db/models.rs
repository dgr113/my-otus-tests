use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, Identifiable };

use crate::db::schema::{ users };




#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String
}
