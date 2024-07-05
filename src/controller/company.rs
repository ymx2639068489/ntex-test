use ntex::web;
use ntex::web::Responder;
use crate::models::param::company::InsertCompanyDTO;
use crate::models::param::QueryUuid;
use crate::models::query::company::QueryCompanyQO;

pub async fn get_page_list(
    pool: crate::Pool,
    pager: web::types::Query<QueryCompanyQO>
) -> impl Responder {
    super::ControllerResponse::ok_pager(
        crate::service::company::get_page_list(pool, pager.into_inner()).await
    )
}

pub async fn update() -> String {
    "chello".to_owned()
}

pub async fn insert(
  pool: crate::Pool,
  target_company: web::types::Json<InsertCompanyDTO>
) -> impl Responder {
    super::ControllerResponse::ok(
      crate::service::company::insert_one_company(pool, target_company.name.clone()).await
    )
}

pub async fn delete(
  pool: crate::Pool,
  target_id: web::types::Query<QueryUuid>
) -> impl Responder {
    super::ControllerResponse::ok(
        crate::service::company::delete_one_company(pool, target_id.into_inner().id).await
    )
}
