extern crate bookish;
extern crate diesel;

use self::diesel::prelude::*;
use self::bookish::*;
use std::env::args;

fn main() {
  use bookish::schema::authors::dsl::*;

  let q = args().nth(1).expect("Expected a pattern to find authors");
  let q = format!("%{}%", q);
  let conn = establish_connection();
  // let n = diesel::delete(authors.filter("first_name like 'foo'"))
  let n = diesel::delete(authors.filter(first_name.like(q)))
    .execute(&conn)
    .expect("Error deleting authors");
  println!("Deleted {} authors", n);
}
