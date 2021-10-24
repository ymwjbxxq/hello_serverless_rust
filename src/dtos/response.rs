use serde::{Serialize};

#[derive(Serialize)]
pub struct Response {
  pub req_id: String,
  pub msg: String,
}