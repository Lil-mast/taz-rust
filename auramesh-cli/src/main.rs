use anyhow::Result;
use auramesh_agents::agents::PlannerAgent;
use auramesh_core::load_config;
use auramesh_mesh::start_mesh;
use clap::{Parser, Subcommand};
use env_logger::init as init_logger;
use log::info;
use tokio::runtime::Runtime;
mod voice;

/// AuraMesh CLI: Offline-first infrastructure healer.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the mesh network.
    StartMesh,
    /// Run a voice command (stub for now).
    Voice { input: String },
    /// Execute a plan.
    Execute { plan: String },
}

fn main() -> Result<()> {
    init_logger();
    let rt = Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config("config.json")?;
    info!("Starting AuraMesh in offline mode: {}", config.offline_mode);

    match cli.command {
        Command::StartMesh => {
            start_mesh().await?;
        }
        Command::Voice { input: _ } => {
            // Use real voice; input as fallback.
            match voice::voice_to_text("path/to/tiny.en.whisper.ggml") {
                Ok(text) => {
                    println!("Transcribed: {}", text);
                }
                Err(e) => {
                    eprintln!("Voice command failed: {}", e);
                    eprintln!("Note: You need to download a Whisper model file (e.g., tiny.en.whisper.ggml)");
                }
            }
        }
        Command::Execute { plan } => {
            let planner = PlannerAgent::new();
            let result = planner.plan(&plan)?;
            println!("Generated Plan: {}", result);
        }
    }
    Ok(())
}

