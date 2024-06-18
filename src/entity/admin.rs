use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::admin)]
#[diesel(belongs_to(crate::schema::role::dsl::role, foreign_key = role_id))]
#[diesel(belongs_to(crate::schema::company::dsl::company, foreign_key = company_id))]
pub struct AdminDO {
    pub id: String,
    pub role_id: String,
    pub company_id: String,
    pub username: String,
    pub password: String,
    pub nickname: String,
}