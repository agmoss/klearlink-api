use rocket::{delete, get, post, put};

use super::dto::ConsumerCreditDto;
use super::service::ConsumerCreditService;
use crate::consumer_credit::dto::ConsumerMatchDto;

use crate::core::auth::AuthResponse;
use crate::core::pool::Db;
use crate::core::response::{RestDto, RestResult};

#[put("/consumer-credit/<consumer_credit_id>", data = "<record>")]
pub async fn submit_consumer_credit<'r>(
    consumer_credit_id: String,
    record: RestDto<'r, ConsumerCreditDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::submit_consumer_credit(consumer_credit_id, record, auth, conn).await
}

#[delete("/consumer-credit/user/<username>")]
pub async fn delete_consumer_credits_by_username(username: String, conn: Db) -> RestResult<()> {
    ConsumerCreditService::delete_consumer_credits_by_username(username, conn).await
}

#[post("/consumer-credit/<consumer_credit_id>", data = "<record>")]
pub async fn update_consumer_credit<'r>(
    consumer_credit_id: String,
    record: RestDto<'r, ConsumerCreditDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::update_consumer_credit(consumer_credit_id, record, auth, conn).await
}

#[get("/consumer-credit/<consumer_credit_id>")]
pub async fn view_consumer_credit(
    consumer_credit_id: String,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::view_consumer_credit(consumer_credit_id, auth, conn).await
}

#[get("/consumer-credit/<consumer_credit_id>/consumer-match")]
pub async fn view_consumer_match(
    consumer_credit_id: String,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<Vec<ConsumerMatchDto>> {
    ConsumerCreditService::view_consumer_match(consumer_credit_id, auth, conn).await
}
