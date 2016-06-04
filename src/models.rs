use diesel::pg::data_types::PgTimestamp;
use super::schema::authors;

#[derive(Queryable)]
pub struct Author {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
  pub middle_name: Option<String>,
}

#[insertable_into(authors)]
pub struct NewAuthor<'a> {
  pub first_name: &'a str,
  pub middle_name: Option<&'a str>,
  pub last_name: &'a str,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
}

#[derive(Queryable)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub published_at: Option<PgTimestamp>,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
}

#[derive(Queryable)]
pub struct Authorship {
  pub id: i32,
  pub author_id: i32,
  pub book_id: i32,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
}
