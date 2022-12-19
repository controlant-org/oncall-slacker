use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmailLookup {
  pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
  pub id: String,
}
