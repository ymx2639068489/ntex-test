use ntex::web;
use ntex::web::Responder;
use crate::models::query::company::QueryCompanyQO;

pub async fn get_page_list(
    pool: crate::Pool,
    pager: web::types::Query<QueryCompanyQO>
) -> impl Responder {
    let res = crate::service::company::get_page_list(pool, pager.into_inner()).await;
    match res {
        Ok(res) => {
            crate::Response::ok_list(res, "获取成功")
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            crate::Response::server_error("error")
        }
    }
}

pub async fn update() -> String {
    "chello".to_owned()
}

pub async fn insert() -> String {
    "chello".to_owned()
}

pub async fn delete() -> String {
    "chello".to_owned()
}
