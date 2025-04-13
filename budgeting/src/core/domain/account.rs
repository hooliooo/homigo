pub mod commands;
pub mod events;

use crate::core::domain::transaction::Transaction;
use ddd::traits::entity::Entity;
use iso_currency::Currency;
use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(ddd::Aggregate, Debug)]
pub struct Account {
    #[generate_id(Uuid)]
    #[entity_id]
    id: AccountId,
    #[field]
    name: String,
    #[field]
    amount: Decimal,
    #[field]
    currency: Currency,
    #[field]
    transactions: HashSet<Transaction>,
}

impl Account {
    fn new(id: Uuid, name: String, amount: Decimal, currency: Currency) -> Self {
        Self {
            id: AccountId::new(id),
            name,
            amount,
            currency,
            transactions: HashSet::default(),
        }
    }
}
