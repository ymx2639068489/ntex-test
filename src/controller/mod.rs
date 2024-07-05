mod base_product;
mod product;
mod company;


use ntex::web::{self, Responder};
use serde::Serialize;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/base_product")
                .route("/get_list", web::get().to(base_product::get_page_list))
                .route("/update", web::put().to(base_product::update))
                .route("/delete", web::delete().to(base_product::delete))
                .route("/insert", web::delete().to(base_product::insert))
        )
        .service(
            web::scope("/product")
                .route("/get_list", web::get().to(product::get_page_list))
                .route("/update", web::put().to(product::update))
                .route("/delete", web::delete().to(product::delete))
                .route("/insert", web::delete().to(product::insert))
        )
        .service(
            web::scope("/company")
                .route("/get_list", web::get().to(company::get_page_list))
                .route("/update", web::put().to(company::update))
                .route("/delete", web::delete().to(company::delete))
                .route("/insert", web::delete().to(company::insert))
        )
    ;
}


pub struct ControllerResponse;

impl ControllerResponse {
    pub fn ok<T: Serialize>(server_res: Result<T, web::Error>) -> impl Responder {
        match server_res {
            Ok(_res) => {
                crate::Response::ok::<T>(None)
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                crate::Response::server_error("error")
            }
        }
    }
    pub fn ok_data<T: Serialize>(server_res: Result<T, web::Error>) -> impl Responder {
        match server_res {
            Ok(res) => {
                crate::Response::ok(Some(res))
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                crate::Response::server_error("error")
            }
        }
    }
    pub fn ok_list<T: Serialize>(server_res: Result<Vec<T>, web::Error>) -> impl Responder {
        match server_res {
            Ok(res) => {
                crate::Response::ok_list(res, "获取成功")
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                crate::Response::server_error("error")
            }
        }
    }
    pub fn ok_pager<T: Serialize>(server_res: Result<(Vec<T>, crate::models::response::Pager), web::Error>) -> impl Responder {
        match server_res {
            Ok(res) => {
                crate::Response::ok_pager(res.0, res.1)
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                crate::Response::server_error("error")
            }
        }
    }
}