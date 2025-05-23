use crate::core::application::account::account_write_repository::AccountWriteRepository;
use crate::core::domain::account::{Account, AccountId};
use async_trait::async_trait;
pub struct SqlAccountWriteRepository;

#[async_trait]
impl AccountWriteRepository for SqlAccountWriteRepository {
    async fn get_by_id(&self, id: AccountId) -> Account {
        todo!()
    }

    async fn create(&self, account: Account) {
        todo!()
    }
}
