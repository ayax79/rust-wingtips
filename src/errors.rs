use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::ParseBoolError;
use std::convert::From;

#[derive(Debug)]
pub enum TracingError {
    MissingFieldError(String),
    SpanParseIntError(ParseIntError),
    SpanParseBoolError(ParseBoolError)
}

impl Error for TracingError {
    fn description(&self) -> &str {
        match *self {
            TracingError::MissingFieldError(_) => "A field was missing",
            TracingError::SpanParseIntError(_) => "A int parse error occurred",
            TracingError::SpanParseBoolError(_) => "A bool parse error occurred"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TracingError::SpanParseIntError(ref error) => Some(error as &Error),
            TracingError::SpanParseBoolError(ref error) => Some(error as &Error),
            _ => None
        }
    }
}

impl Display for TracingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<ParseIntError> for TracingError {
    fn from(err: ParseIntError) -> TracingError {
        TracingError::SpanParseIntError(err)
    }
}

impl From<ParseBoolError> for TracingError {
    fn from(err: ParseBoolError) -> TracingError {
        TracingError::SpanParseBoolError(err)
    }
}