use ntex::web;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;
pub type Pool = web::types::State<DbPool>;
pub type Conn = diesel::MysqlConnection;