#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::people;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub profession: Option<String>,
    pub salary: Option<i32>,
}