use crate::core::domain::category::Category;
use chrono::{DateTime, Utc};
use ddd::{Entity, structs::invariant_error::InvariantError};
use ddd::{structs::error_detail::ErrorDetail, traits::entity::Entity};
use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Entity, Debug)]
pub struct Transaction {
    #[entity_id]
    id: Uuid,
    #[field]
    purpose: String,
    #[field]
    amount: Decimal,
    #[field]
    date_time: DateTime<Utc>,
    #[field]
    category: Option<Category>,
    #[field]
    subtransactions: HashSet<Transaction>,
}

impl Transaction {
    fn new(
        id: Uuid,
        purpose: String,
        amount: Decimal,
        date_time: DateTime<Utc>,
        category: Option<Category>,
        subtransactions: HashSet<Transaction>,
    ) -> Self {
        Self {
            id,
            purpose,
            amount,
            date_time,
            category,
            subtransactions,
        }
    }

    fn new_now(id: Uuid, purpose: String, amount: Decimal) -> Self {
        Self::new(id, purpose, amount, Utc::now(), None, HashSet::default())
    }

    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), InvariantError> {
        if self.subtransactions().contains(&transaction) {
            return Err(Error::AlreadyAdded(transaction.id).to_invariant_error());
        }
        let _result = self.subtransactions.insert(transaction);
        #[cfg(test)]
        {
            let msg = format!("Insertion result: {:?}", _result);
            dbg!(msg);
        }
        Ok(())
    }

    fn remove_transaction_by_id(&mut self, id: &Uuid) -> Result<(), InvariantError> {
        match self.subtransactions.remove(id) {
            true => Ok(()),
            false => Err(Error::NotFound(*id).to_invariant_error()),
        }
    }
}

impl std::borrow::Borrow<Uuid> for Transaction {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}

enum Error {
    AlreadyAdded(Uuid),
    NotFound(Uuid),
}

impl Error {
    fn to_invariant_error(&self) -> InvariantError {
        match self {
            Self::AlreadyAdded(id) => InvariantError::new(HashSet::from([ErrorDetail::new(
                "error.transaction.already-added".into(),
                format!("transaction with {id} already added as subtransaction"),
            )])),

            Self::NotFound(id) => InvariantError::new(HashSet::from([ErrorDetail::new(
                "error.transaction.not-found".into(),
                format!("Transaction with {id} was not found"),
            )])),
        }
    }
}

#[cfg(test)]
mod test {

    mod constructor {
        use rust_decimal::Decimal;
        use uuid::Uuid;

        use crate::core::domain::transaction::Transaction;

        #[test]
        fn given_a_transaction_when_removing_a_subtransaction_then_the_correct_result_should_be_returned()
         {
            let mut transaction = Transaction::new_now(
                Uuid::new_v4(),
                "Test transaction".into(),
                Decimal::new(1000, 2),
            );

            let subtransaction_id = Uuid::new_v4();
            let other_transaction =
                Transaction::new_now(subtransaction_id, String::default(), Decimal::new(1000, 2));
            let _ = transaction.add_transaction(other_transaction);
            assert_eq!(transaction.subtransactions().len(), 1);

            let result = transaction.remove_transaction_by_id(&subtransaction_id);
            assert!(result.is_ok());
            assert!(transaction.subtransactions().is_empty());

            let result = transaction.remove_transaction_by_id(&subtransaction_id);
            assert!(result.is_err());

            let error = result
                .unwrap_err()
                .error_details()
                .clone()
                .into_iter()
                .next()
                .unwrap();

            assert_eq!(
                error.message(),
                &format!("Transaction with {subtransaction_id} was not found")
            )
        }
    }
}
