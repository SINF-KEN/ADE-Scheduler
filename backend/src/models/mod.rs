use super::schema::users;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}
