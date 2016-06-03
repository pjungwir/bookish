extern crate bookish;
extern crate diesel;

use self::diesel::prelude::*;
use self::bookish::*;
use self::bookish::models::Author;
use std::env::args;

fn main() {
  use bookish::schema::authors::dsl::{authors, published};

  let id = args().nth(1).expect("publish_author requires an author id")
    .parse::<i32>().expect("invalid id");
  let conn = establish_connection();

  let a = diesel::update(authors.find(id))
    .set(published.eq(true))
    .get_result::<Author>(&conn)
    .expect(&format!("Unable to find author {}", id));
  println!("Published author {} {}", a.first_name, a.last_name);
}
