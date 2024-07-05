use serde::Deserialize;

pub mod company;


#[derive(Debug, Deserialize)]
pub struct QueryUuid {
  pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryIntid {
  pub id: i64,
}

