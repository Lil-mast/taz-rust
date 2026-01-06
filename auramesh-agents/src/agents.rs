use anyhow::{Context, Result};
use log::info;

// Trait for all agents, inspired by MCP: Standardized execute with context.
pub trait Agent {
    fn name(&self) -> &str;
    fn execute(&self, input: &str, context: &str) -> Result<String>;  // Context param for MCP-like passing
}

/// Planner Agent: NL to DAG.
/// MVP: Rule-based; later LLM. Uses context for RAG/MCP.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(&self, nl_input: &str) -> Result<String> {
        info!("Planning for: {}", nl_input);
        // Stub: Simple rules.
        let dag = if nl_input.contains("scale") {
            r#"{"steps": [{"action": "scale_workers", "deps": []}]}"#.to_string()
        } else {
            r#"{"steps": [{"action": "check_status", "deps": []}]}"#.to_string()
        };
        Ok(dag).context("Planning failed")
    }
}

impl Agent for PlannerAgent {
    fn name(&self) -> &str {
        "Planner"
    }
    fn execute(&self, input: &str, _context: &str) -> Result<String> {
        self.plan(input)
    }
}

pub struct InfraAgent;
impl Agent for InfraAgent {
    fn name(&self) -> &str {
        "Infra"
    }
    fn execute(&self, _input: &str, _context: &str) -> Result<String> {
        Ok("Executed infra action".to_string())
    }
}