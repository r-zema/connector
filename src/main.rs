mod api_gateway;
mod config;
mod translator;
mod routers;

use config::IntegrationConfig;
use api_gateway::gateway::ApiGateway;
use std::{thread, time::Duration};

fn main() {
    let config = IntegrationConfig::new();
    let api_gateway = ApiGateway::new(config);

    println!("Application started!");

    // Keep the application running
    loop {
        thread::sleep(Duration::from_secs(60));  // Sleep for 60 seconds
    }
}