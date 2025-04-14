use crate::core::domain::account::{Account, AccountId};

#[async_trait::async_trait]
pub trait AccountWriteRepository: Send + Sync {
    /// Queries an [`Account`] by id
    /// # Arguments
    /// * `id` - The id of the [`Account`]
    /// # Returns
    /// An existing [`Account`] with the specified id
    async fn get_by_id(&self, id: AccountId) -> Account;
    /// Creates an Account for the User
    /// # Arguments
    /// * `account` - The [`Account`] to be created
    async fn create(&self, account: Account);

}