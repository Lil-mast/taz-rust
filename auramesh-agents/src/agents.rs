use anyhow::Result;
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
        // Enhanced rule-based.
        let steps = if nl_input.contains("scale") && nl_input.contains("database") {
            r#"{"steps": [{"action": "check_db_load"}, {"action": "scale_db", "deps": ["check_db_load"]}]}"#
        } else {
            r#"{"steps": [{"action": "default_check"}]}"#
        };
        Ok(steps.to_string())
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

impl InfraAgent {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_dag(&self, dag_json: &str) -> Result<()> {
        info!("Executing DAG: {}", dag_json);
        let docker = bollard::Docker::connect_with_local_defaults()?;
        // MVP: Create hello-world container. MCP: This could be a protocol call to 'exec://docker'.
        let _ = docker.create_container::<&str, &str>(None, bollard::container::Config {
            image: Some("hello-world"),
            ..Default::default()
        }).await?;
        // Systemd stub: std::process::Command::new("systemctl").arg("restart").arg("service");
        Ok(())
    }
}

impl Agent for InfraAgent {
    fn name(&self) -> &str {
        "Infra"
    }
    fn execute(&self, input: &str, _context: &str) -> Result<String> {
        tokio::runtime::Handle::current().block_on(self.execute_dag(input))?;
        Ok("Executed".to_string())
    }
}