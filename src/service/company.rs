use crate::models::entity::company::{CompanyEntity};
use crate::models::query::company::QueryCompanyQO;

use ntex::web;

pub async fn get_page_list(
    pool: crate::Pool,
    pager: QueryCompanyQO
) -> Result<Vec<CompanyEntity>, web::Error> {
    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");
    let company_list = web::block(move || crate::dao::company::query_page_company(&mut conn, &pager))
        .await
        .map_err(web::error::ErrorInternalServerError)?;

    Ok(company_list)
}

pub async fn insert_one_company(pool: crate::Pool, company_name: String) -> Result<bool, web::Error> {

    let target_company = CompanyEntity::new(company_name);

    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");
    let result = web::block(move || crate::dao::company::insert_one_company(&mut conn, target_company))
        .await
        .map_err(web::error::ErrorInternalServerError)?;

    Ok(result)
}