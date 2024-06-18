use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// 允许查询、select，序列化，插入和修改操作
#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = crate::schema::company)]
pub struct CompanyDO {
    pub id: String,
    pub name: String,
}
impl CompanyDO {
    pub fn new(name: String) -> CompanyDO {
        CompanyDO {
            id: uuid::Uuid::new_v4().to_string(),
            name
        }
    }
}