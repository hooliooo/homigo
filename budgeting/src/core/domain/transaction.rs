use crate::core::domain::category::Category;
use chrono::{DateTime, Utc};
use ddd::Entity;
use ddd::traits::entity::Entity;
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
    fn new_now(id: Uuid, purpose: String, amount: Decimal) -> Self {
        Self {
            id,
            purpose,
            amount,
            date_time: Utc::now(),
            category: None,
            subtransactions: HashSet::default(),
        }
    }
}
