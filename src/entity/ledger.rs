use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Debug, Clone, Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::ledger)]
#[diesel(belongs_to(crate::schema::product::dsl::product, foreign_key = id))]
pub struct LedgerDTO {
    pub id: String,
    pub product_id: String,
    pub product_name: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub people_number: i32,
    pub product_type: String,
    pub duration: i32,
    pub revenue: BigDecimal,
    pub cost: BigDecimal,
    pub pay_status: String,
    pub executor: String,
    pub notes: Option<String>,
}