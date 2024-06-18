use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// 完整的结构体
#[derive(Debug, Clone, Queryable, Serialize, Selectable)]
#[diesel(table_name = crate::schema::custom)]
pub struct CustomDTO {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub id_type: Option<String>,
    pub id_number: Option<String>,
    pub level: i32,
}