
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDTO {
    a: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCompanyDTO {
  pub name: String,
}