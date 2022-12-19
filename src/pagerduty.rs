use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Schedule {
  pub oncalls: Vec<OnCall>,
}

#[derive(Deserialize, Debug)]
pub struct OnCall {
  pub user: SchedUser,
}

#[derive(Deserialize, Debug)]
pub struct SchedUser {
  #[serde(rename = "self")]
  pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
  pub user: UserInner,
}

#[derive(Deserialize, Debug)]
pub struct UserInner {
  pub email: String,
}
