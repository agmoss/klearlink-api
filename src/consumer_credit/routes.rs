use rocket::{delete, get, post, put};

use super::dto::{
    ConsumerCreditDto, ConsumerMatchDto, InsertConsumerCreditDto, UpdateConsumerCreditDto,
};
use super::service::ConsumerCreditService;

use crate::core::auth::AuthResponse;
use crate::core::pool::Db;
use crate::core::req::RestDto;
use crate::core::res::RestResult;

/// Route for adding a consumer credit record
///
/// To use this route, use a PUT request. The ID of the consumer credit to be created
/// should also be passed through the URL.
///
/// Upon success, returns the newly created consumer credit record in JSON format
#[put("/consumer-credit/<consumer_credit_id>", data = "<record>")]
pub async fn submit_consumer_credit<'r>(
    consumer_credit_id: String,
    record: RestDto<'r, InsertConsumerCreditDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::submit_consumer_credit(consumer_credit_id, record, auth, conn).await
}

/// Route for updating a previously submitted consumer credit record.
///
/// The ID of the consumer credit record to update should be passed through the route.
///
/// Upon success, returns the updated consumer credit record in JSON format
#[post("/consumer-credit/<consumer_credit_id>", data = "<record>")]
pub async fn update_consumer_credit<'r>(
    consumer_credit_id: String,
    record: RestDto<'r, UpdateConsumerCreditDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::update_consumer_credit(consumer_credit_id, record, auth, conn).await
}

/// Route for viewing a previously submitted consumer credit record.
///
/// The ID of the consumer credit record to update should be passed through the route.
///
/// Upon success, returns the consumer credit record in JSON format
#[get("/consumer-credit/<consumer_credit_id>")]
pub async fn view_consumer_credit(
    consumer_credit_id: String,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    ConsumerCreditService::view_consumer_credit(consumer_credit_id, auth, conn).await
}

/// Route for viewing a consumer match for a consumer credit record.
///
/// The ID of the consumer credit record to match should be passed through the route.
///
/// Upon success, returns the consumer match record in JSON format
#[get("/consumer-credit/<consumer_credit_id>/consumer-match")]
pub async fn view_consumer_match(
    consumer_credit_id: String,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<ConsumerMatchDto> {
    ConsumerCreditService::view_consumer_match(consumer_credit_id, auth, conn).await
}

#[delete("/consumer-credit/user/<username>")]
pub async fn delete_consumer_credits_by_username(
    username: String,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<()> {
    match auth {
        Ok(user) => {
            user.ensure_admin()?;
            ConsumerCreditService::delete_consumer_credits_by_username(username, conn).await
        }
        Err(err) => Err(err),
    }
}
