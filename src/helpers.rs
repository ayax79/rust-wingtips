use errors::TracingError;
use std::str::FromStr;
use std::convert::From;

use std::collections::HashMap;

pub trait GetAs {

    fn get_as_str(&self, k: &str) -> Result<&str, TracingError>;

    fn get_as_u64(&self, k: &str) -> Result<u64, TracingError>;

    fn get_as_string(&self, k: &str) -> Result<String, TracingError>;

    fn get_as_bool(&self, k: &str) -> Result<bool, TracingError>;


}

impl<'a> GetAs for HashMap<&'a str, &'a str> {

    fn get_as_str(&self, k: &str) -> Result<&str, TracingError> {
        let msg = "missing field: ".to_string() + k;

        self.get(k)
            .map(|v| *v)
            .ok_or(TracingError::MissingFieldError(msg))
    }

    fn get_as_string(&self, k: &str) -> Result<String, TracingError> {
        self.get_as_str(k)
            .map(|v| v.to_string())
    }


    fn get_as_u64(&self, k: &str) -> Result<u64, TracingError> {
        self.get_as_str(k).and_then(|v| FromStr::from_str(v).map_err(|e| From::from(e)))
    }

    fn get_as_bool(&self, k: &str) -> Result<bool, TracingError> {
        self.get_as_str(k).and_then(|v| FromStr::from_str(v).map_err(|e| From::from(e)))
    }

}

