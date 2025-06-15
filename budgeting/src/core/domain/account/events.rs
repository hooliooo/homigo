use ddd::{
    enums::environment::Environment,
    structs::ids::{CommandId, EventId, IssuerId},
};
use iso_currency::Currency;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::AccountId;

#[derive(ddd::DomainEvent, ddd::ValueObject)]
pub struct CreatedAccount {
    #[field]
    id: AccountId,
    #[field]
    name: String,
    #[field]
    amount: Decimal,
    #[field]
    currency: Currency,
    command_id: CommandId,
    environment: Environment,
    event_id: EventId,
    issuer_id: IssuerId,
    issued_at: chrono::DateTime<chrono::Utc>,
}

impl CreatedAccount {
    pub fn new(
        id: AccountId,
        name: String,
        amount: Decimal,
        currency: Currency,
        command_id: uuid::Uuid,
    ) -> Self {
        Self {
            id,
            name,
            amount,
            currency,
            command_id: CommandId::new(command_id),
            environment: ddd::enums::environment::Environment::Development,
            event_id: EventId::new_random(),
            issuer_id: IssuerId::new(Uuid::new_v4()),
            issued_at: chrono::Utc::now(),
        }
    }
}
