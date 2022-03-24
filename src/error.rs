use actix_web::{HttpResponse, http::StatusCode};
use serde::{Serialize, Deserialize};

type ErrorSpec = (&'static str, &'static str);

pub const BLOB_UNKNOWN: ErrorSpec =
    ("BLOB_UNKNOWN", "blob unknown to registry"); 
pub const MANIFEST_UNKNOWN: ErrorSpec =
    ("MANIFEST_UNKNOWN", "manifest unknown to registry");
pub const UNKNOWN_ERROR: ErrorSpec =
    ("UNKNOWN ERROR", "something is very wrong");

/// Error type expected by OCI specification
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryError {
    /// The code field MUST be a unique identifier, containing only uppercase
    /// alphabetic characters and underscores. 
    code: String,
    /// The message field is OPTIONAL, and if present, it SHOULD be a human
    /// readable string or MAY be empty.
    message: String,
    /// The detail field is OPTIONAL and MAY contain arbitrary JSON data
    /// providing information the client can use to resolve the issue.
    detail: Detail,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Detail {
    reason: String
}

/// Registry spec for error response includes an array of errors
/// TODO: Some way to construct a multi-error
#[derive(Serialize)]
struct RegistryErrorResponse {
    errors: Vec<RegistryError>
}

impl RegistryError {
    pub fn from(spec: ErrorSpec) -> RegistryError {
        RegistryError {
            code: String::from(spec.0),
            message: String::from(spec.1),
            detail: Detail { reason: String::from("") }
        }
    }

    pub fn from_err(spec: ErrorSpec, err: Box<dyn std::error::Error>) -> RegistryError {
        RegistryError {
            code: String::from(spec.0),
            message: String::from(spec.1),
            detail: Detail { reason: String::from(err.to_string()) }
        }
    }

    // Convert the error to an HttpResponse for Actix
    pub fn respond(&self) -> HttpResponse {
        let response = RegistryErrorResponse {
            errors: vec![self.clone()]
        };
        let payload = serde_json::to_vec(&response).unwrap();
        HttpResponse::build(StatusCode::NOT_FOUND)
            .append_header(("Content-Type", "application/json"))
            .body(payload)
    }
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_an_error() {
        let err = RegistryError::from(BLOB_UNKNOWN);
        assert_eq!(err.code, "BLOB_UNKNOWN");
    }
}
