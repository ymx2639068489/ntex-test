mod base_product;
mod product;

use ntex::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/base_product")
                .route("/get_list", web::get().to(base_product::get_page_list))
                .route("/update", web::put().to(base_product::update))
                .route("/delete", web::delete().to(base_product::delete))
                .route("/insert", web::delete().to(base_product::insert))
        ).service(
            web::scope("/product")
                .route("/get_list", web::get().to(product::get_page_list))
                .route("/update", web::put().to(base_product::update))
                .route("/delete", web::delete().to(base_product::delete))
                .route("/insert", web::delete().to(base_product::insert))
        )
    ;
}