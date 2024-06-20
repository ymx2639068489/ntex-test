use diesel::prelude::*;
use serde::Serialize;

// 完整的结构体
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::custom)]
pub struct CustomEntity {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub id_type: Option<String>,
    pub id_number: Option<String>,
    pub level: i32,
}