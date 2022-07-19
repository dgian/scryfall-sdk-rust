//! Module containing definitions for error object(s)

use std::error::Error;
use std::fmt::Display;
use crate::resources::ResourceKind;
use serde::{Deserialize, Serialize};

/// Error response body.
/// 
/// See more info in [Scryfall API official documentation](https://scryfall.com/docs/api/errors) 
/// about the error object.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorBody {
    pub code: String,
    
    pub details: String,
    
    pub error_type: Option<String>,
    
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    
    pub status: i16,
    
    pub warnings: Option<Vec<String>>,
}

impl Display for ErrorBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}: {}", self.code, self.details)
    }
}

impl Error for ErrorBody { }

impl ErrorBody {
    /// Transforms an reqwest::Error to ErrorBody
    /// 
    /// These errors will include for example connection errors,
    /// or json decoding errors.
    /// 
    /// A default specific error body is defined for these kind of errors:
    /// 
    /// ```json
    /// {
    ///     "code": "CLIENT_ERR",
    ///     "details": Actual description from reqwest error,
    ///     "kind": "error",
    ///     "status": 599
    /// }
    /// ```
    pub fn from_reqwest_error(e: reqwest::Error) -> Self {
        ErrorBody {
            code: "CLIENT_ERR".into(),
            details: e.to_string(),
            error_type: None,
            kind: ResourceKind::Error,
            status: 599,
            warnings: None,
        }
    }
}