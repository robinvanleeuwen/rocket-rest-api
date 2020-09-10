#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::people;
use crate::people::Person;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Person>> {
    people::table.load::<Person>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Person> {
    people::table.find(id).get_result::<Person>(connection)
}

pub fn insert(person: Person, connection: &PgConnection) -> QueryResult<Person> {
    diesel::insert_into(people::table)
        .values(&InsertablePerson::from_person(person))
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(connection)
}


#[derive(Insertable)]
#[table_name = "people"]
struct InsertablePerson {
    first_name: String,
    last_name: String,
    age: Option<i32>,
    profession: Option<String>,
    salary: Option<i32>,
}

impl InsertablePerson {
    fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
            age: person.age,
            profession: person.profession,
            salary: person.salary,
        }
    }
}