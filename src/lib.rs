#![feature(non_ascii_idents)]
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;


#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate time;

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

  let t = time::get_time();
  // Postgres expects *micro*seconds.
  // Also its Epoch starts Jan 1, 2000,
  // but the Unix Epoch starts Jan 1, 1970.
  let μs = 1_000_000 * t.sec + (t.nsec as i64 / 1000);
  let μs = μs - 946684800000000;  // 2000-01-01 in μs
  // println!("{}", μs);

  let a = NewAuthor {
    first_name: first_name,
    last_name: last_name,
    created_at: PgTimestamp(μs),
    updated_at: PgTimestamp(μs),
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
