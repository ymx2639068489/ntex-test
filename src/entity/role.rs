use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = crate::schema::role)]
pub struct RoleDO {
    pub id: String,
    #[diesel(column_name = "rolename")]
    pub role_name: String,
    pub description: String,
    pub router: String,
    pub admin_value: i32,
    pub operator_value: i32,
    pub role_value: i32,
    pub company_value: i32,
    pub salesman_value: i32,
    pub base_product_value: i32,
    pub product_value: i32,
    pub sales_records_value: i32,
    pub custom_value: i32,
    pub ledger_value: i32,
}