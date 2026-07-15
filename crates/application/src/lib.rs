//! Application use cases and transport-neutral data structures.

use domain::{Greeting, GreetingName, GreetingNameError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateGreeting {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GreetingResponse {
    pub message: String,
}

pub fn create_greeting(command: CreateGreeting) -> Result<GreetingResponse, GreetingNameError> {
    let name = GreetingName::new(command.name)?;
    let greeting = Greeting::new(name);

    Ok(GreetingResponse {
        message: greeting.message(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_greeting() {
        let response = create_greeting(CreateGreeting {
            name: "Ada".to_owned(),
        })
        .unwrap();

        assert_eq!(response.message, "Hello, Ada!");
    }
}
