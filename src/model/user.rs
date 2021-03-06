use chrono;
use serde_derive::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub birthday: Option<chrono::NaiveDate>,
}

#[derive(Clone, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub birthday: Option<chrono::NaiveDate>,
}