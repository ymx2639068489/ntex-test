
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::product)]
pub struct ProductEntity {
    pub id: String,
    pub base_product_id: String,
    pub create_at: chrono::NaiveDateTime,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub people_number: i32,
    pub duration: i32,
}