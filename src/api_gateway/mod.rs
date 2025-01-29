//module declarations, public exports

pub mod gateway;
pub mod authentication;
pub mod authorisation;
pub mod validation;

// Re-export main types for easier access
pub use gateway::ApiGateway;
pub use authentication::{Authenticator, AuthenticationError};
pub use authorisation::{Authoriser, AuthorisationError};
pub use validation::{Validator, ValidationError};