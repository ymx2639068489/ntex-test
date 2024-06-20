use crate::models::query::Pager;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryCompanyQO {
    pub page: i64,
    pub page_size: i64,
    pub name: Option<String>,
    pub id: Option<String>,
}

impl Pager for QueryCompanyQO {
    fn get_page(&self) -> i64 {
        self.page
    }
    fn get_page_size(&self) -> i64 {
        self.page_size
    }
}