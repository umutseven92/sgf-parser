use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SgfParseError {
    details: String,
}

impl SgfParseError {
    pub fn new(details: String) -> Self {
        SgfParseError { details }
    }
}

impl Display for SgfParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for SgfParseError {}
