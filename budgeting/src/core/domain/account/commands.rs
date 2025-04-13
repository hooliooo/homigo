use chrono::{DateTime, Utc};
use ddd::{
    Request, ValueObject, enums::environment::Environment, structs::ids::RequestId,
};
use iso_currency::Currency;
use rust_decimal::Decimal;

use super::AccountId;

#[derive(Request, ValueObject)]
pub struct CreateAccount {
    #[field]
    id: AccountId,
    #[field]
    name: String,
    #[field]
    amount: Decimal,
    #[field]
    currency: Currency,
    request_id: RequestId,
    environment: Environment,
    issuer_id: (),
    issued_at: DateTime<Utc>,
}
