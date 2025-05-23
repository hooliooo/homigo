use std::sync::Arc;

use account_write_repository::AccountWriteRepository;
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
    type Response = ();

    fn handle(&self, request: CreateAccount) -> impl Future<Output = Self::Response> + Send {
        let result: Result<Account, ()> = request.try_into();
        match result {
            Ok(account) => account.create(),
            Err(error) => {
                panic!("Error, could not create account")
            }
        };
        async {}
    }
}
