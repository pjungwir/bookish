#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::pg::data_types::PgTimestamp;
use dotenv::dotenv;
use std::env;
use self::models::{Author, NewAuthor};

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set.");
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_author<'a>(conn: &PgConnection,
                         first_name: &'a str,
                         last_name: &'a str) -> Author {
  use schema::authors;

  let a = NewAuthor {
    first_name: first_name,
    last_name: last_name,
    created_at: PgTimestamp(500736408041057),
    updated_at: PgTimestamp(500736408041057),
  };

  diesel::insert(&a).into(authors::table)
    .get_result(conn)
    .expect("Error saving new author")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
