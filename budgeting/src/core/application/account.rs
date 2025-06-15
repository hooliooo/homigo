use std::sync::Arc;

use account_write_repository::AccountWriteRepository;
use ddd::structs::invariant_error::InvariantError;
use ddd::traits::request::Request;
use ddd::traits::use_case::UseCase;

use crate::core::domain::account::Account;
use crate::core::domain::{
    account::commands::CreateAccount, services::authorization_service::AuthorizationService,
};

pub mod account_write_repository;

#[derive(Clone)]
pub struct AddAccountUseCase {
    authorization_service: AuthorizationService,
    repository: Arc<dyn AccountWriteRepository>,
}

impl UseCase for AddAccountUseCase {
    type Request = CreateAccount;
    type Response = Result<(), InvariantError>;

    async fn handle(&self, request: CreateAccount) -> Self::Response {
        let request_id = *request.request_id();
        let account: Account = request.try_into()?;
        let event = account.create(request_id);
        Ok(())
    }
}
