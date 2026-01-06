use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};

/// core configuration structure for AuraMesh.
/// Incorpates MCP principles: Standardized context for model-tool interactions.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub offline_mode: bool,
    pub data_dir: String,
    // MCP stub: Context protocol fields for agent-tool handshakes.
    pub mcp_endpoints: Vec<String>, // E.g, local URIs for tools like "file;//docs" or "exec://docker"
}

impl Default for Config {
    fn default() -> Self {
        Self {
            offline_mode: true,
            data_dir: "./data".to_string(),
            mcp_endpoints: vec![],
        }
    }
}

/// Load config from file or defaults.
/// Offline-first: No network calls, per MCP local resilience.

pub fn load_config(path: &str) -> Result<Config> {
    info!("Loading config from {}", path);
    // Stub: In real impl, read from JSON. MCP could extend to dynamic context loading.
    Ok(Config::default()).context("Failed to load config")
}