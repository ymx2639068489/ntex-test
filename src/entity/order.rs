use diesel::prelude::*;
use bigdecimal::BigDecimal;
// use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::custom_salesman)]
#[diesel(belongs_to(crate::schema::custom::dsl::custom, foreign_key = custom_id))]
#[diesel(belongs_to(crate::schema::salesman::dsl::salesman, foreign_key = salesman_id))]
#[diesel(belongs_to(crate::schema::product::dsl::product, foreign_key = product_id))]
pub struct OrderDO {
    pub id: i32,
    pub custom_id: i32,
    pub salesman_id: i32,
    pub product_id: String,
    pub create_at: chrono::NaiveDateTime,
    pub company: String,
    pub order_id: String,
    pub pay_method: String,
    pub money: BigDecimal,
    pub people_number: i32,
    pub rebate: Option<String>,
}