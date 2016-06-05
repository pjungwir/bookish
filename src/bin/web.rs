extern crate dotenv;
extern crate iron;

use iron::prelude::*;
use iron::status;
use std::env;
use dotenv::dotenv;

fn main() {
  fn authors(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
  }

  dotenv().ok();
  let port = env::var("BOOKISH_PORT").expect("Please set BOOKISH_PORT");
  let addr = &*format!("localhost:{}", port);
  println!("Listening on {}", port);
  Iron::new(authors).http(addr).unwrap();
}
