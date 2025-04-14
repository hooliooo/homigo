use std::sync::Arc;

use account_write_repository::AccountWriteRepository;
use ddd::traits::use_case::UseCase;

use crate::core::domain::{account::commands::CreateAccount, services::authorization_service::{self, AuthorizationService}};

pub mod account_write_repository;

#[derive(Clone)]
pub struct AddAccountUseCase {
    authorization_service: AuthorizationService,
    repository: Arc<dyn AccountWriteRepository>
}

impl UseCase for AddAccountUseCase {
    type Request = CreateAccount;
    type Response = ();

    fn handle(&self, request: Self::Request) -> impl Future<Output = Self::Response> + Send {
        async {}
    }
}
