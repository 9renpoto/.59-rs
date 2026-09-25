//! Common command error handling for the desktop adapter.

use domain::GreetingNameError;
use serde::Serialize;

/// Common error response for Tauri commands.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum CommandError {
    /// Validation or business rule violation error.
    Validation(String),
    /// Unexpected or internal error.
    Internal(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(msg) => write!(f, "Validation error: {msg}"),
            Self::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<GreetingNameError> for CommandError {
    fn from(err: GreetingNameError) -> Self {
        match err {
            GreetingNameError::Empty => Self::Validation("名前を入力してください。".to_owned()),
            GreetingNameError::TooLong => {
                Self::Validation("名前が長すぎます（100文字以内で入力してください）。".to_owned())
            }
        }
    }
}
