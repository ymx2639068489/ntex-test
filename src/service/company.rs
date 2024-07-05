use crate::models::entity::company::{CompanyEntity};
use crate::models::query::company::QueryCompanyQO;
use crate::models::response::Pager;

use ntex::web;

pub async fn get_page_list(
    pool: crate::Pool,
    pager: QueryCompanyQO
) -> Result<(Vec<CompanyEntity>, Pager), web::Error> {

    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");

    Ok(
        web::block(move || crate::dao::company::query_page_company(&mut conn, &pager))
            .await
            .map_err(web::error::ErrorInternalServerError)?
    )
}

pub async fn insert_one_company(pool: crate::Pool, company_name: String) -> Result<bool, web::Error> {

    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");

    Ok(
        web::block(move || crate::dao::company::insert_one_company(&mut conn,  &CompanyEntity::new(company_name)))
            .await
            .map_err(web::error::ErrorInternalServerError)?
    )
}

pub async fn delete_one_company(pool: crate::Pool, id: String) -> Result<bool, web::Error> {
    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");

    Ok(
        web::block(move || crate::dao::company::delete_company(&mut conn,  &id))
            .await
            .map_err(web::error::ErrorInternalServerError)?
    )
}