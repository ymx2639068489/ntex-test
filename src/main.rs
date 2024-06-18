mod config;
mod controller;

use ntex::web;
use diesel::{r2d2, MysqlConnection};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

#[ntex::main]
async fn main() -> std::io::Result<()> {


    let pool = init_env_and_get_dp();
    let port = 8080;
    println!("🚀 Server started successfully: http://0.0.0.0:{}", port);

    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .wrap(web::middleware::Logger::default())
            .wrap(ntex_cors::Cors::default())
            .configure(controller::init_routes)
    })
    .workers(2)
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
fn init_env_and_get_dp() -> DbPool {
    // 环境变量
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool: DbPool = r2d2::Pool::builder()
        .build(r2d2::ConnectionManager::<MysqlConnection>::new(&database_url))
        .expect("database url error");
    pool
}