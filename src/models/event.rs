use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable,AsChangeset};

#[derive(Queryable, Serialize, Deserialize,Debug,Clone,AsChangeset,Insertable)]
#[diesel(table_name=crate::repository::schema::events)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Deserialize, Serialize,Debug,Clone,Insertable)]
#[diesel(table_name=crate::repository::schema::events)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
    pub location: String,
}
