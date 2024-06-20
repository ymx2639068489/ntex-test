use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::operator)]
pub struct OperatorEntity {
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