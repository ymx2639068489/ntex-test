use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable)]
pub struct OperatorDO {
    pub id: i32,
    pub admin_id: String,
    pub teablename: String,
    pub source_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub operator_type: String,
    pub origin_object: Option<String>,
    pub now_object: Option<String>,
    pub notes: String,
}