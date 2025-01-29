#[derive(Clone)]
pub struct IntegrationConfig {
    pub vi: ViConfig,
    pub mas: MasConfig,
    pub maximo: MaximoConfig,
}

#[derive(Clone)]
pub struct ViConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: u64,
}

#[derive(Clone)]
pub struct MasConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: u64,
}

#[derive(Clone)]
pub struct MaximoConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: u64,
}

impl IntegrationConfig {
    pub fn new() -> Self {
        Self {
            vi: ViConfig {
                base_url: "http://vi-service".to_string(),  // Hardcoded default values
                api_key: "test-key".to_string(),
                timeout: 5000,
            },
            mas: MasConfig {
                base_url: "http://mas-service".to_string(),
                api_key: "test-key".to_string(),
                timeout: 5000,
            },
            maximo: MaximoConfig {
                base_url: "http://maximo-service".to_string(),
                api_key: "test-key".to_string(),
                timeout: 5000,
            },
        }
    }
}