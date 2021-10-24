use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Request {
  pub name: String,
}
