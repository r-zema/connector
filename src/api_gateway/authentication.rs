//authentication and authorization logic for incoming requests
//VI side - I know every request is valid

use crate::config::IntegrationConfig;

// Add this at the top
use super::gateway::ApiRequest;  // Import ApiRequest type

#[derive(Debug)]
pub enum AuthenticationError {
    InvalidApiKey(String),
    ExpiredToken(String),
    MalformedToken(String),
}

pub struct Authenticator {
    config: IntegrationConfig,
}

impl Authenticator {
    pub fn new(config: IntegrationConfig) -> Self {
        Self { config }
    }

    // Add this method
    pub fn authenticate(&self, request: &ApiRequest) -> Result<(), AuthenticationError> {
        // For now, just return Ok to get it compiling
        Ok(())
        // TODO: Implement real authentication using verify_token
    }

    pub fn verify_token(&self, token: &str) -> Result<(), AuthenticationError> {
        // Authentication logic here
        todo!()
    }
}