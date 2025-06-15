pub mod commands;
pub mod events;

use super::validate_whitespace;
use crate::core::domain::account::events::CreatedAccount;
use crate::core::domain::transaction::Transaction;
use crate::core::domain::{ResultValidation, account::commands::CreateAccount};
use ddd::structs::ids::RequestId;
use ddd::structs::invariant_error::InvariantError;
use ddd::traits::entity::Entity;
use iso_currency::Currency;
use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;
use validator::Validate;

#[derive(ddd::Aggregate, Debug, Validate)]
pub struct Account {
    #[generate_id(Uuid)]
    #[entity_id]
    id: AccountId,
    #[field]
    #[validate(
        custom(function = "validate_whitespace"),
        length(max = 20, message = "must be 20 or less chars")
    )]
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
    fn try_new(
        id: Uuid,
        name: String,
        amount: Decimal,
        currency: Currency,
    ) -> Result<Account, InvariantError> {
        let account = Self {
            id: AccountId::new(id),
            name,
            amount,
            currency,
            transactions: HashSet::default(),
        };

        account.validate().map(|_| account).transform_errors()
    }

    pub fn create(&self, request_id: RequestId) -> CreatedAccount {
        CreatedAccount::new(
            self.id,
            self.name.clone(),
            self.amount,
            self.currency,
            *request_id.value(),
        )
    }
}

impl TryFrom<CreateAccount> for Account {
    type Error = InvariantError;

    fn try_from(value: CreateAccount) -> Result<Self, Self::Error> {
        Self::try_new(value.id.value(), value.name, value.amount, value.currency)
    }
}

#[cfg(test)]
mod test {
    mod constructor {
        use iso_currency::Currency;
        use rust_decimal::Decimal;
        use uuid::Uuid;

        use crate::core::domain::account::Account;

        #[test]
        fn given_an_invalid_name_when_instantiating_then_it_should_be_an_error() {
            let result =
                Account::try_new(Uuid::new_v4(), "".into(), Decimal::default(), Currency::EUR);
            assert!(result.is_err());
            let details = result.err().unwrap().error_details().clone();
            assert_eq!(details.len(), 1);
            dbg!(details.clone());
            let detail = details.get("error.account.invalid-name").unwrap();
            assert_eq!("'name' is blank", detail.message());

            let result = Account::try_new(
                Uuid::now_v7(),
                "a".repeat(21),
                Decimal::default(),
                Currency::EUR,
            );
            assert!(result.is_err());
            let details = result.err().unwrap().error_details().clone();
            assert_eq!(details.len(), 1);
            let detail = details.get("error.account.invalid-name").unwrap();
            assert_eq!("'name' must be 20 or less chars", detail.message());
        }

        #[test]
        fn given_a_valid_name_when_instantiating_then_it_should_be_successful() {
            let result = Account::try_new(
                Uuid::new_v4(),
                "Test Account".into(),
                Decimal::default(),
                Currency::EUR,
            );
            assert!(result.is_ok())
        }
    }

    mod create {
        use ddd::traits::{entity::Entity, request::Request};
        use iso_currency::Currency;
        use rust_decimal::Decimal;
        use uuid::Uuid;

        use crate::core::domain::account::{Account, AccountId, commands::CreateAccount};

        #[test]
        fn given_an_account_when_create_is_called_then_the_event_should_match() {
            let command = CreateAccount::new(
                AccountId::new(Uuid::now_v7()),
                "Test Account".into(),
                Decimal::default(),
                Currency::EUR,
            );
            let result = Account::try_new(
                command.id().value(),
                command.name().clone(),
                *command.amount(),
                *command.currency(),
            );

            let account = result.unwrap();
            let event = account.create(*command.request_id());
            assert_eq!(account.id(), event.id());
            assert_eq!(account.name(), event.name());
            assert_eq!(account.amount(), event.amount());
            assert_eq!(account.currency(), event.currency());
        }
    }
}
