use ddd::traits::use_case::UseCase;

pub struct AddAccountUseCase;

impl UseCase for AddAccountUseCase {
    type Request = ();
    type Response = ();

    fn handle(&self, request: Self::Request) -> impl Future<Output = Self::Response> + Send {
        async {}
    }
}
