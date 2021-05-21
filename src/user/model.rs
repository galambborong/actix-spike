use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub name: String,
  // _age: u8,
}
