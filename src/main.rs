mod executor;
mod server;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

use server::AgentBrowserServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Verify agent-browser is available
    if !executor::check_agent_browser().await {
        eprintln!(
            "Warning: agent-browser CLI not found. \
             Install it with: cargo install agent-browser && agent-browser install"
        );
    }

    let server = AgentBrowserServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    // Clean up browser daemon on shutdown
    executor::close_browser().await;

    Ok(())
}
