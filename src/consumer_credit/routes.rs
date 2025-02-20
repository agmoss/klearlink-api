use rocket::{get, post, put};

use super::dto::ConsumerCreditDto;
use crate::consumer_credit::dto::ConsumerMatchDto;
use crate::consumer_credit::service::{
    submit_consumer_credit_service, update_consumer_credit_service, view_consumer_credit_service,
    view_consumer_match_service,
};
use crate::core::auth::ApiKeyAuth;
use crate::core::pool::Db;
use crate::core::response::{RestDto, RestResult};

#[put("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
pub async fn submit_consumer_credit<'r>(
    consumer_credit_id_dto: String,
    record: RestDto<'r, ConsumerCreditDto>,
    _auth: ApiKeyAuth,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    submit_consumer_credit_service(consumer_credit_id_dto, record, _auth.user_id, conn).await
}

#[post("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
pub async fn update_consumer_credit<'r>(
    consumer_credit_id_dto: String,
    record: RestDto<'r, ConsumerCreditDto>,
    _auth: ApiKeyAuth,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    update_consumer_credit_service(consumer_credit_id_dto, record, conn).await
}

#[get("/consumer-credit/<consumer_credit_id_dto>")]
pub async fn view_consumer_credit(
    consumer_credit_id_dto: String,
    _auth: ApiKeyAuth,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    view_consumer_credit_service(consumer_credit_id_dto, conn).await
}

#[get("/consumer-credit/<consumer_credit_id_dto>/consumer-match")]
pub async fn view_consumer_match(
    consumer_credit_id_dto: String,
    _auth: ApiKeyAuth,
    conn: Db,
) -> RestResult<Vec<ConsumerMatchDto>> {
    view_consumer_match_service(consumer_credit_id_dto, _auth.user_id, conn).await
}
