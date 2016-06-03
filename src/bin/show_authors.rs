extern crate bookish;
extern crate diesel;

use self::bookish::*;
use self::bookish::models::*;
use self::diesel::prelude::*;

fn main() {
  use bookish::schema::authors::dsl::*;

  let conn = establish_connection();
  let results = authors.filter(published.eq(true))
    .limit(5)
    .load::<Author>(&conn)
    .expect("Error loading authors");

  println!("Displaying {} authors", results.len());
  for a in results {
    println!("{}: {} {}", a.id, a.first_name, a.last_name);
  }
}
