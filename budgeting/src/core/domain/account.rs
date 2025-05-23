pub mod commands;
pub mod events;

use crate::core::domain::account::commands::CreateAccount;
use crate::core::domain::account::events::CreatedAccount;
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
    /// Creates a new instance of an Account
    ///
    /// `id`      : The unique identifier for the Account
    /// `name`    : The name of the Account
    /// `amount`  : The amount in the Account
    /// `currency`: The currency of the Account
    fn new(id: Uuid, name: String, amount: Decimal, currency: Currency) -> Self {
        Self {
            id: AccountId::new(id),
            name,
            amount,
            currency,
            transactions: HashSet::default(),
        }
    }

    pub fn create(&self) -> CreatedAccount {
        todo!()
    }
}

impl TryFrom<CreateAccount> for Account {
    type Error = ();

    fn try_from(value: CreateAccount) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            amount: value.amount,
            currency: value.currency,
            transactions: HashSet::default(),
        })
    }
}
