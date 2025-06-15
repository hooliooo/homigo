use chrono::{DateTime, Utc};
use ddd::{Request, ValueObject, enums::environment::Environment, structs::ids::RequestId};
use iso_currency::Currency;
use rust_decimal::Decimal;

use super::AccountId;

#[derive(Request, ValueObject)]
pub struct CreateAccount {
    #[field]
    pub id: AccountId,
    #[field]
    pub name: String,
    #[field]
    pub amount: Decimal,
    #[field]
    pub currency: Currency,
    request_id: RequestId,
    environment: Environment,
    issuer_id: (),
    issued_at: DateTime<Utc>,
}

impl CreateAccount {
    pub fn new(id: AccountId, name: String, amount: Decimal, currency: Currency) -> Self {
        Self {
            id,
            name,
            amount,
            currency,
            request_id: RequestId::new_random(),
            environment: Environment::Development,
            issuer_id: (),
            issued_at: Utc::now(),
        }
    }
}
