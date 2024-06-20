use diesel::internal::table_macro::BoxedSelectStatement;
use diesel::mysql::Mysql;

pub mod company;

fn apply_filter<T, U, F>(
    query: BoxedSelectStatement<'_, T, U, Mysql>,
    filter: Option<F>,
    apply_fn: impl Fn(BoxedSelectStatement<'_, T, U, Mysql>, F) -> BoxedSelectStatement<'_, T, U, Mysql>
) -> BoxedSelectStatement<'_, T, U, Mysql> {
    if let Some(value) = filter {
        return apply_fn(query, value);
    }
    query
}

fn diesel_res_usize_to_bool(res: Result<usize, diesel::result::Error>) -> Result<bool, diesel::result::Error> {
    match res {
        Ok(size) => Ok(size != 0),
        Err(e) => Err(e),
    }
}