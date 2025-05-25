pub mod account;
pub mod category;
pub mod services;
pub mod transaction;

use ddd::structs::error_detail::ErrorDetail;
use ddd::structs::invariant_error::InvariantError;
use ddd::traits::aggregate::Aggregate;
use std::collections::HashSet;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

trait ResultValidation<T: Aggregate> {
    fn transform_errors(self) -> Result<T, InvariantError>;
}

impl<T: Aggregate> ResultValidation<T> for Result<T, ValidationErrors> {
    fn transform_errors(self) -> Result<T, InvariantError> {
        self.map_err(|err| {
            let errors = err
                .0
                .into_iter()
                .filter_map(|(key, value)| {
                    if let ValidationErrorsKind::Field(errors) = value {
                        Some((key, errors))
                    } else {
                        None
                    }
                })
                .flat_map(|(key, errors)| {
                    let type_name = T::type_name();
                    let error_key = {
                        let mut error_key = String::with_capacity(16 + type_name.len() + key.len());
                        error_key.push_str("error.");
                        error_key.push_str(type_name);
                        error_key.push_str(".invalid-");
                        error_key
                            .extend(key.as_ref().chars().map(|c| if c == '_' { '-' } else { c }));
                        error_key
                    };
                    errors.into_iter().filter_map(move |error| {
                        error.message.as_deref().map(|message| {
                            ErrorDetail::new(error_key.clone(), format!("'{}' {}", key, message))
                        })
                    })
                })
                .collect::<HashSet<ErrorDetail>>();
            InvariantError::new(errors)
        })
    }
}
