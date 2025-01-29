use super::authentication::{Authenticator, AuthenticationError};
use super::authorisation::{Authoriser, AuthorisationError};
use super::validation::{Validator, ValidationError};
use crate::config::IntegrationConfig;

pub type ApiRequest = String;
pub type ApiResponse = String;

// Simple error enum that wraps our other errors
#[derive(Debug)]
pub enum GatewayError {
    Validation(String),
    Authentication(String),
    Authorisation(String),
}

pub struct ApiGateway {
    authenticator: Authenticator,
    authoriser: Authoriser,
    validator: Validator,
}

impl ApiGateway {
    pub fn new(config: IntegrationConfig) -> Self {
        Self {
            authenticator: Authenticator::new(config.clone()),
            authoriser: Authoriser::new(),
            validator: Validator::new(),
        }
    }

    pub fn process_request(&self, request: ApiRequest) -> Result<ApiResponse, GatewayError> {
        // For now, just return Ok to get it compiling
        Ok("Response processed".to_string())
    }
}