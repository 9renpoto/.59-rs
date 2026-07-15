//! Framework-independent domain rules for the sample application.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreetingName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GreetingNameError {
    Empty,
    TooLong,
}

impl GreetingName {
    pub const MAX_LENGTH: usize = 100;

    pub fn new(value: impl Into<String>) -> Result<Self, GreetingNameError> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(GreetingNameError::Empty);
        }

        if value.chars().count() > Self::MAX_LENGTH {
            return Err(GreetingNameError::TooLong);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeting {
    name: GreetingName,
}

impl Greeting {
    pub fn new(name: GreetingName) -> Self {
        Self { name }
    }

    pub fn message(&self) -> String {
        format!("Hello, {}!", self.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_a_valid_name() {
        assert_eq!(GreetingName::new("  Ada  ").unwrap().as_str(), "Ada");
    }

    #[test]
    fn rejects_an_empty_name() {
        assert_eq!(GreetingName::new("  "), Err(GreetingNameError::Empty));
    }
}
