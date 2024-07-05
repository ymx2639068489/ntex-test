use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// 允许查询、select，序列化，插入和修改操作
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = crate::schema::company)]
pub struct CompanyEntity {
    pub id: String,
    pub name: String,
}
impl CompanyEntity {
    pub fn new(name: String) -> CompanyEntity {
        CompanyEntity {
            id: uuid::Uuid::new_v4().to_string(),
            name,
        }
    }
}