use crate::config::IntegrationConfig;
use crate::translator::mas::translator::MasTranslator;
use crate::translator::maximo::translator::MaximoTranslator;

#[derive(Debug)]
pub enum RouterError {
    InvalidRoute(String),
    TranslationError(String),
    SystemUnavailable(String),
    UnknownSystem(String),
}

#[derive(Debug, Clone, Copy)]  // Add Copy trait
pub enum SystemType {
    VI,
    MAS,
    MAXIMO,
}

pub struct ApiRouter {
    config: IntegrationConfig,
    mas_translator: MasTranslator,
    maximo_translator: MaximoTranslator,
}

pub struct RouteRequest {
    pub source_system: SystemType,
    pub target_system: SystemType,
    pub payload: String,  // Could be generic over payload type
}

pub struct RouteResponse {
    pub status: RouteStatus,
    pub translated_payload: String,  // Could be generic over payload type
}

pub enum RouteStatus {
    Success,
    Error(String),
}

impl ApiRouter {
    pub fn new(config: IntegrationConfig) -> Self {
        Self {
            mas_translator: MasTranslator::new(config.clone()),
            maximo_translator: MaximoTranslator::new(config.clone()),
            config,
        }
    }

    pub async fn route(&self, request: RouteRequest) -> Result<RouteResponse, RouterError> {
        match (request.source_system, request.target_system) {
            (SystemType::VI, SystemType::MAS) => {
                self.handle_vi_to_mas(request).await
            },
            (SystemType::MAS, SystemType::VI) => {
                self.handle_mas_to_vi(request).await
            },
            (SystemType::VI, SystemType::MAXIMO) => {    // Changed from Maximo to MAXIMO
                self.handle_vi_to_maximo(request).await
            },
            (SystemType::MAXIMO, SystemType::VI) => {    // Changed from Maximo to MAXIMO
                self.handle_maximo_to_vi(request).await
            },
            _ => Err(RouterError::InvalidRoute("Unsupported route".into())),
        }
    }

    async fn handle_vi_to_mas(&self, request: RouteRequest) -> Result<RouteResponse, RouterError> {
        // Route to MAS translator
        todo!()
    }

    async fn handle_mas_to_vi(&self, request: RouteRequest) -> Result<RouteResponse, RouterError> {
        // Route to MAS translator for reverse translation
        todo!()
    }

    async fn handle_vi_to_maximo(&self, request: RouteRequest) -> Result<RouteResponse, RouterError> {
        // Route to Maximo translator
        todo!()
    }

    async fn handle_maximo_to_vi(&self, request: RouteRequest) -> Result<RouteResponse, RouterError> {
        // Route to Maximo translator for reverse translation
        todo!()
    }
}