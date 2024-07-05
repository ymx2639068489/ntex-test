
use diesel::prelude::*;
use crate::models::entity::company::*;
use crate::models::query::company::*;
use crate::models::query::Pager;
use crate::models::response;
use super::{apply_filter, diesel_res_usize_to_bool};

/// 获取分页后的公司
pub fn query_page_company(conn: &mut crate::Conn, pager: &QueryCompanyQO) -> QueryResult<(Vec<CompanyEntity>, response::Pager)> {
    
    let get_sql = |pager: &QueryCompanyQO| {
        use crate::schema::company::dsl::*;
        // 获取动态query
        let query = company.into_boxed();
        // like name
        let query = apply_filter(query, pager.name.to_owned(),|q, v| q.filter(name.like(v)));
        // eq id
        let query = apply_filter(query, pager.id.to_owned(), |q, v| q.filter(id.eq(v)));
        query
    };

    let total = get_sql(pager).count().get_result(conn).expect("");

    // 分页
    let data: Vec<CompanyEntity> = get_sql(pager)
        .limit(pager.get_limit())
        .offset(pager.get_offset())
        .load::<CompanyEntity>(conn)?;

    Ok((data, response::Pager::new(pager, total)))
}

/// 插入一条公司记录
pub fn insert_one_company(conn: &mut crate::Conn, target_company: &CompanyEntity) -> QueryResult<bool> {
    use crate::schema::company::dsl::*;
    diesel_res_usize_to_bool(diesel::insert_into(company)
        .values(target_company)
        .execute(conn)
    )
}
/// 删除一条公司记录
/// 如果有关联记录的话 应该会报错
pub fn delete_company(conn: &mut crate::Conn, target_company_id: &String) -> QueryResult<bool> {
    use crate::schema::company::dsl::*;
    let target = company.filter(id.eq(target_company_id));
    diesel_res_usize_to_bool(diesel::delete(target).execute(conn))
}
