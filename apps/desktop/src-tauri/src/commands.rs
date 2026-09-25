//! Tauri command handlers.

use crate::error::CommandError;
use application::{create_greeting, CreateGreeting, GreetingResponse};

/// Handler for creating a greeting using the shared application use case.
#[tauri::command]
pub fn greet(name: String) -> Result<GreetingResponse, CommandError> {
    create_greeting(CreateGreeting { name }).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_greeting_successfully() {
        let response = greet("Ada".to_owned()).expect("expected successful greeting");
        assert_eq!(response.message, "Hello, Ada!");
    }

    #[test]
    fn returns_validation_error_on_empty_name() {
        let err = greet("   ".to_owned()).expect_err("expected error for empty name");
        assert_eq!(
            err,
            CommandError::Validation("名前を入力してください。".to_owned())
        );
    }
}
