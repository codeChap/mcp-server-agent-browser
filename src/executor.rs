use std::time::Duration;
use tokio::process::Command;

const DEFAULT_TIMEOUT_SECS: u64 = 60;

/// Path to the agent-browser executable, overridable via env var.
fn agent_browser_path() -> String {
    std::env::var("AGENT_BROWSER_PATH").unwrap_or_else(|_| "agent-browser".into())
}

/// Validate a session ID to prevent CLI argument injection.
/// Allows alphanumeric, hyphens, underscores, and dots only.
pub fn validate_session_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("Session ID cannot be empty".into());
    }
    if id.len() > 128 {
        return Err("Session ID too long (max 128 chars)".into());
    }
    if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        return Err(format!(
            "Invalid session ID '{id}': only alphanumeric, hyphens, underscores, and dots allowed"
        ));
    }
    Ok(())
}

/// Validate a file path to prevent path traversal attacks.
pub fn validate_file_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("File path cannot be empty".into());
    }
    // Reject path traversal
    for component in std::path::Path::new(path).components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(format!(
                "Invalid file path '{path}': path traversal ('..') is not allowed"
            ));
        }
    }
    // Reject paths to sensitive system directories
    let sensitive = ["/etc", "/usr", "/boot", "/sys", "/proc", "/dev", "/sbin", "/lib"];
    for prefix in sensitive {
        if path.starts_with(prefix) {
            return Err(format!(
                "Invalid file path '{path}': writing to '{prefix}' is not allowed"
            ));
        }
    }
    Ok(())
}

/// Execute an agent-browser CLI command and return stdout.
pub async fn exec_browser(args: Vec<String>, timeout_secs: Option<u64>) -> Result<String, String> {
    let bin = agent_browser_path();

    let mut cmd = Command::new(&bin);
    cmd.args(&args);
    cmd.env("NO_COLOR", "1");
    cmd.kill_on_drop(true);

    let timeout = Duration::from_secs(timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS));

    let output = tokio::time::timeout(timeout, cmd.output())
        .await
        .map_err(|_| format!("agent-browser timed out after {}s", timeout.as_secs()))?
        .map_err(|e| format!("Failed to execute agent-browser: {e}"))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string();
        if stdout.is_empty() {
            Ok("Command executed successfully".into())
        } else {
            Ok(stdout)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string();
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

/// Check if agent-browser is available on the system.
pub async fn check_agent_browser() -> bool {
    let bin = agent_browser_path();
    let timeout = Duration::from_secs(5);
    let mut cmd = Command::new(&bin);
    cmd.arg("--version");
    cmd.kill_on_drop(true);

    tokio::time::timeout(timeout, cmd.output())
        .await
        .ok()
        .and_then(|r| r.ok())
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Close the browser daemon gracefully.
pub async fn close_browser() {
    let bin = agent_browser_path();
    let mut cmd = Command::new(&bin);
    cmd.arg("close");
    cmd.env("NO_COLOR", "1");
    cmd.kill_on_drop(true);

    let _ = tokio::time::timeout(Duration::from_secs(10), cmd.output()).await;
}
