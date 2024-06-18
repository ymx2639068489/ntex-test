use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::base_product)]
pub struct BaseProductDO {
    pub id: String,
    pub name: Option<String>,
    pub create_at: chrono::NaiveDateTime,
    #[diesel(column_name = type)]
    pub product_type: String,
    pub file: Option<String>,
    pub notes: Option<String>,
}