extern crate bookish;
extern crate diesel;

use self::bookish::*;
use std::io::stdin;

fn main() {
  let conn = establish_connection();

  println!("What is the first name?");
  let mut first_name = String::new();
  stdin().read_line(&mut first_name).unwrap();
  let first_name = &first_name[..(first_name.len() - 1)]; // drop newline
  
  println!("last name?");
  let mut last_name = String::new();
  stdin().read_line(&mut last_name).unwrap();
  let last_name = &last_name[..(last_name.len() - 1)]; // drop newline

  let a = create_author(&conn, first_name, last_name);
  println!("Saved author with id {}", a.id);

}


