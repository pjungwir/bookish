use diesel::pg::data_types::PgTimestamp;
use super::schema::authors;

#[derive(Queryable)]
pub struct Author {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub published: bool,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
}

#[insertable_into(authors)]
pub struct NewAuthor<'a> {
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub created_at: PgTimestamp,
  pub updated_at: PgTimestamp,
}
