use crate::core::application::account::account_write_repository::AccountWriteRepository;
use crate::core::domain::account::{Account, AccountId};

pub struct AccountWriteRepositorySql;

impl AccountWriteRepository for AccountWriteRepositorySql {
    async fn get_by_id(&self, id: AccountId) -> Account {
        todo!()
    }

    async fn create(&self, account: Account) {
        todo!()
    }
}