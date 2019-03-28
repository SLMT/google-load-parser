
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GoogleLoadParseError {
    description: String
}

impl GoogleLoadParseError {
    pub fn new_boxed(description: String) -> Box<GoogleLoadParseError> {
        return Box::new(GoogleLoadParseError {
            description: description
        });
    }
}

impl fmt::Display for GoogleLoadParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error: {}", self.description)
    }
}

impl Error for GoogleLoadParseError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}