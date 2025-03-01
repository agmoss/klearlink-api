use super::{
    pool::Db,
    response::{BaseResponse, ErrorResponse, RestResult},
};

pub async fn execute_db_operation_rest<T, F, R>(
    conn: &Db,
    db_op: F,
    success_handler: impl Fn(T) -> RestResult<R>,
) -> RestResult<R>
where
    F: FnOnce(&mut diesel::PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let result = conn.run(db_op).await;
    match result {
        Ok(value) => success_handler(value),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

pub async fn execute_db_operation<T, F, R>(
    conn: &Db,
    db_op: F,
    success_handler: impl Fn(T) -> BaseResponse<R>,
) -> BaseResponse<R>
where
    F: FnOnce(&mut diesel::PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let result = conn.run(db_op).await;
    match result {
        Ok(value) => success_handler(value),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}
