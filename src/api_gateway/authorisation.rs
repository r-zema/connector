use super::gateway::ApiRequest;

#[derive(Debug)]
pub enum AuthorisationError {
    InsufficientPermissions(String),
    InvalidRole(String),
    ResourceAccessDenied(String),
}

pub struct Authoriser {
    // Authorization-specific fields
}

impl Authoriser {
    pub fn new() -> Self {
        Self {}
    }

    // Add this method
    pub fn authorise(&self, request: &ApiRequest) -> Result<(), AuthorisationError> {
        // For now, just return Ok to get it compiling
        Ok(())
        // TODO: Implement real authorization using check_permissions
    }

    pub fn check_permissions(&self, source: &str, action: &str) -> Result<(), AuthorisationError> {
        // Authorisation logic
        todo!()
    }
}