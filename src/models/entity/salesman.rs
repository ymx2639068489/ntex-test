use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::salesman)]
#[diesel(belongs_to(crate::schema::company::dsl::company, foreign_key = company_id))]
pub struct SalesmanEntity {
    pub id: i32,
    pub company_id: String,
    pub username: String,
    pub phone: String,
}