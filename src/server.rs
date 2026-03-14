use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::executor::{exec_browser, validate_file_path, validate_session_id};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn run(args: Vec<String>) -> Result<CallToolResult, McpError> {
    run_with_timeout(args, None).await
}

async fn run_with_timeout(args: Vec<String>, timeout_secs: Option<u64>) -> Result<CallToolResult, McpError> {
    match exec_browser(args, timeout_secs).await {
        Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
    }
}

fn validated_path(path: &str) -> Result<(), McpError> {
    validate_file_path(path).map_err(|e| McpError::invalid_params(e, None))
}

fn session_args(session_id: Option<&str>) -> Result<Vec<String>, McpError> {
    match session_id {
        Some(sid) => {
            validate_session_id(sid).map_err(|e| {
                McpError::invalid_params(e, None)
            })?;
            Ok(vec!["--session".into(), sid.into()])
        }
        None => Ok(vec![]),
    }
}

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SessionOnly {
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UrlParams {
    #[schemars(description = "The URL to navigate to")]
    pub url: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectorParams {
    #[schemars(description = "CSS selector, text, or @ref from snapshot")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectorValueParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Value to enter")]
    pub value: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TypeTextParams {
    #[schemars(description = "Selector for the input element")]
    pub selector: String,
    #[schemars(description = "Text to type")]
    pub text: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyParams {
    #[schemars(description = "Key to press (e.g. 'Enter', 'Escape', 'Tab', 'Control+a')")]
    pub key: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyboardTypeParams {
    #[schemars(description = "Text to type with real keystrokes")]
    pub text: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollParams {
    #[schemars(description = "Scroll direction")]
    pub direction: ScrollDirection,
    #[schemars(description = "Scroll amount in pixels")]
    pub amount: Option<u32>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DragParams {
    #[schemars(description = "Selector for source element")]
    pub source: String,
    #[schemars(description = "Selector for destination element")]
    pub destination: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UploadParams {
    #[schemars(description = "Selector for the file input element")]
    pub selector: String,
    #[schemars(description = "File paths to upload")]
    pub files: Vec<String>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DownloadParams {
    #[schemars(description = "Selector for the element to click to trigger download")]
    pub selector: String,
    #[schemars(description = "File path to save the download")]
    pub path: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OptionalSelectorParams {
    #[schemars(description = "Selector for the element (full page if not provided)")]
    pub selector: Option<String>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetHtmlParams {
    #[schemars(description = "Selector for the element (full page if not provided)")]
    pub selector: Option<String>,
    #[schemars(description = "Get outer HTML instead of inner HTML")]
    pub outer: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAttrParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Name of the attribute to get")]
    pub attribute: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SnapshotParams {
    #[schemars(description = "Only show interactive elements")]
    pub interactive: Option<bool>,
    #[schemars(description = "Remove empty structural elements")]
    pub compact: Option<bool>,
    #[schemars(description = "Limit tree depth")]
    pub depth: Option<u32>,
    #[schemars(description = "Scope to a CSS selector")]
    pub selector: Option<String>,
    #[schemars(description = "Return JSON output")]
    pub json: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScreenshotParams {
    #[schemars(description = "File path to save the screenshot")]
    pub path: Option<String>,
    #[schemars(description = "Selector for element to screenshot")]
    pub selector: Option<String>,
    #[schemars(description = "Capture the full scrollable page")]
    pub full_page: Option<bool>,
    #[schemars(description = "Add numbered labels for vision models")]
    pub annotate: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PdfParams {
    #[schemars(description = "File path to save the PDF")]
    pub path: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewSessionParams {
    #[schemars(description = "Viewport width in pixels")]
    pub width: Option<u32>,
    #[schemars(description = "Viewport height in pixels")]
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloseSessionParams {
    #[schemars(description = "Session ID to close")]
    pub session_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WaitState {
    Attached,
    Detached,
    Visible,
    Hidden,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WaitParams {
    #[schemars(description = "Selector to wait for, or milliseconds (e.g. '5000')")]
    pub selector: String,
    #[schemars(description = "Maximum wait time in milliseconds")]
    pub timeout: Option<u64>,
    #[schemars(description = "Element state to wait for")]
    pub state: Option<WaitState>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CookieGetParams {
    #[schemars(description = "URLs to get cookies for")]
    pub urls: Option<Vec<String>>,
    #[schemars(description = "Return JSON output")]
    pub json: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CookieSetParams {
    #[schemars(description = "Cookies to set as JSON")]
    pub cookies: serde_json::Value,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EvalParams {
    #[schemars(description = "JavaScript code to execute")]
    pub script: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConnectParams {
    #[schemars(description = "CDP port number or WebSocket URL")]
    pub target: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkParams {
    #[schemars(description = "Return JSON output")]
    pub json: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Server
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct AgentBrowserServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AgentBrowserServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────

    #[tool(description = "Navigate to a URL in the browser")]
    async fn browser_navigate(
        &self,
        Parameters(p): Parameters<UrlParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["open".into(), p.url];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Navigate back in browser history")]
    async fn browser_go_back(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["back".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Navigate forward in browser history")]
    async fn browser_go_forward(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["forward".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Reload the current page")]
    async fn browser_reload(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["reload".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Interaction ─────────────────────────────────────────────────────

    #[tool(description = "Click on an element by selector or @ref from snapshot")]
    async fn browser_click(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["click".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Double-click on an element")]
    async fn browser_dblclick(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["dblclick".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Fill a text input field (clears existing value first)")]
    async fn browser_fill(
        &self,
        Parameters(p): Parameters<SelectorValueParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["fill".into(), p.selector, p.value];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Type text character by character (triggers key events)")]
    async fn browser_type(
        &self,
        Parameters(p): Parameters<TypeTextParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["type".into(), p.selector, p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Type text with real keystrokes without targeting an element")]
    async fn browser_keyboard_type(
        &self,
        Parameters(p): Parameters<KeyboardTypeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keyboard".into(), "type".into(), p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Insert text without key events (no selector needed)")]
    async fn browser_keyboard_inserttext(
        &self,
        Parameters(p): Parameters<KeyboardTypeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keyboard".into(), "inserttext".into(), p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Press a keyboard key")]
    async fn browser_press(
        &self,
        Parameters(p): Parameters<KeyParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["press".into(), p.key];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Hover over an element")]
    async fn browser_hover(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["hover".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Focus an element")]
    async fn browser_focus(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["focus".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Scroll the page in a direction")]
    async fn browser_scroll(
        &self,
        Parameters(p): Parameters<ScrollParams>,
    ) -> Result<CallToolResult, McpError> {
        let dir = match p.direction {
            ScrollDirection::Up => "up",
            ScrollDirection::Down => "down",
            ScrollDirection::Left => "left",
            ScrollDirection::Right => "right",
        };
        let mut a = vec!["scroll".into(), dir.into()];
        if let Some(px) = p.amount {
            a.push(px.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Scroll an element into view")]
    async fn browser_scroll_into_view(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["scrollintoview".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Select an option from a dropdown")]
    async fn browser_select(
        &self,
        Parameters(p): Parameters<SelectorValueParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["select".into(), p.selector, p.value];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check a checkbox or radio button")]
    async fn browser_check(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["check".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Uncheck a checkbox")]
    async fn browser_uncheck(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["uncheck".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Drag an element to another element")]
    async fn browser_drag(
        &self,
        Parameters(p): Parameters<DragParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["drag".into(), p.source, p.destination];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Upload files to a file input element")]
    async fn browser_upload(
        &self,
        Parameters(p): Parameters<UploadParams>,
    ) -> Result<CallToolResult, McpError> {
        for f in &p.files {
            validated_path(f)?;
        }
        let mut a = vec!["upload".into(), p.selector];
        a.extend(p.files);
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Download a file by clicking an element")]
    async fn browser_download(
        &self,
        Parameters(p): Parameters<DownloadParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["download".into(), p.selector, p.path];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Information Retrieval ───────────────────────────────────────────

    #[tool(description = "Get text content from an element or the entire page")]
    async fn browser_get_text(
        &self,
        Parameters(p): Parameters<OptionalSelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "text".into()];
        if let Some(sel) = p.selector {
            a.push(sel);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get HTML content from an element or the entire page")]
    async fn browser_get_html(
        &self,
        Parameters(p): Parameters<GetHtmlParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "html".into()];
        if let Some(sel) = p.selector {
            a.push(sel);
        }
        if p.outer.unwrap_or(false) {
            a.push("--outer".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get an attribute value from an element")]
    async fn browser_get_attribute(
        &self,
        Parameters(p): Parameters<GetAttrParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "attr".into(), p.selector, p.attribute];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get the current page URL")]
    async fn browser_get_url(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "url".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get the current page title")]
    async fn browser_get_title(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "title".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get an accessibility tree snapshot of the page with @refs for AI interaction")]
    async fn browser_snapshot(
        &self,
        Parameters(p): Parameters<SnapshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["snapshot".into()];
        if p.interactive.unwrap_or(false) {
            a.push("-i".into());
        }
        if p.compact.unwrap_or(false) {
            a.push("-c".into());
        }
        if let Some(d) = p.depth {
            a.push("-d".into());
            a.push(d.to_string());
        }
        if let Some(sel) = p.selector {
            a.push("-s".into());
            a.push(sel);
        }
        if p.json.unwrap_or(false) {
            a.push("--json".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Element State ───────────────────────────────────────────────────

    #[tool(description = "Check if an element is visible")]
    async fn browser_is_visible(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "visible".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check if an element is enabled")]
    async fn browser_is_enabled(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "enabled".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check if a checkbox/radio is checked")]
    async fn browser_is_checked(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "checked".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Screenshot & PDF ────────────────────────────────────────────────

    #[tool(description = "Take a screenshot of the page or an element")]
    async fn browser_screenshot(
        &self,
        Parameters(p): Parameters<ScreenshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["screenshot".into()];
        if let Some(ref path) = p.path {
            validated_path(path)?;
            a.push(path.clone());
        }
        if let Some(sel) = p.selector {
            a.push("--selector".into());
            a.push(sel);
        }
        if p.full_page.unwrap_or(false) {
            a.push("--full".into());
        }
        if p.annotate.unwrap_or(false) {
            a.push("--annotate".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Generate a PDF of the current page")]
    async fn browser_pdf(
        &self,
        Parameters(p): Parameters<PdfParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["pdf".into(), p.path];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Session Management ──────────────────────────────────────────────

    #[tool(description = "Create a new isolated browser session")]
    async fn browser_new_session(
        &self,
        Parameters(p): Parameters<NewSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["session".into(), "new".into()];
        if let Some(w) = p.width {
            a.push("--width".into());
            a.push(w.to_string());
        }
        if let Some(h) = p.height {
            a.push("--height".into());
            a.push(h.to_string());
        }
        run(a).await
    }

    #[tool(description = "Close a specific browser session by its ID (use browser_close to close the entire browser)")]
    async fn browser_close_session(
        &self,
        Parameters(p): Parameters<CloseSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        validate_session_id(&p.session_id).map_err(|e| {
            McpError::invalid_params(e, None)
        })?;
        run(vec![
            "close".into(),
            "--session".into(),
            p.session_id,
        ]).await
    }

    // ── Wait ────────────────────────────────────────────────────────────

    #[tool(description = "Wait for an element to appear, or for a number of milliseconds")]
    async fn browser_wait(
        &self,
        Parameters(p): Parameters<WaitParams>,
    ) -> Result<CallToolResult, McpError> {
        // Derive executor timeout from wait timeout so long waits aren't killed prematurely
        let executor_timeout = p.timeout.map(|ms| (ms / 1000) + 10);
        let mut a = vec!["wait".into(), p.selector];
        if let Some(t) = p.timeout {
            a.push("--timeout".into());
            a.push(t.to_string());
        }
        if let Some(s) = p.state {
            let state_str = match s {
                WaitState::Attached => "attached",
                WaitState::Detached => "detached",
                WaitState::Visible => "visible",
                WaitState::Hidden => "hidden",
            };
            a.push("--state".into());
            a.push(state_str.into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run_with_timeout(a, executor_timeout).await
    }

    // ── Cookies ─────────────────────────────────────────────────────────

    #[tool(description = "Get cookies from the browser")]
    async fn browser_get_cookies(
        &self,
        Parameters(p): Parameters<CookieGetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "get".into()];
        if p.json.unwrap_or(false) {
            a.push("--json".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        if let Some(urls) = p.urls {
            a.push("--".into());
            a.extend(urls);
        }
        run(a).await
    }

    #[tool(description = "Set cookies in the browser")]
    async fn browser_set_cookies(
        &self,
        Parameters(p): Parameters<CookieSetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "set".into(), p.cookies.to_string()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Clear all cookies")]
    async fn browser_clear_cookies(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "clear".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── JavaScript ──────────────────────────────────────────────────────

    #[tool(description = "Execute JavaScript in the browser context and return the result")]
    async fn browser_evaluate(
        &self,
        Parameters(p): Parameters<EvalParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["eval".into(), p.script];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Console & Network ───────────────────────────────────────────────

    #[tool(description = "Get console log messages from the browser")]
    async fn browser_get_console(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["console".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get network requests captured by the browser")]
    async fn browser_get_network(
        &self,
        Parameters(p): Parameters<NetworkParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "requests".into()];
        if p.json.unwrap_or(false) {
            a.push("--json".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Connect ─────────────────────────────────────────────────────────

    #[tool(description = "Connect to a running browser via Chrome DevTools Protocol")]
    async fn browser_connect(
        &self,
        Parameters(p): Parameters<ConnectParams>,
    ) -> Result<CallToolResult, McpError> {
        run(vec!["connect".into(), p.target]).await
    }

    // ── Close ───────────────────────────────────────────────────────────

    #[tool(description = "Close the entire browser daemon and all sessions")]
    async fn browser_close(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["close".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }
}

#[tool_handler]
impl ServerHandler for AgentBrowserServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "agent-browser",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Browser automation MCP server. Wraps Vercel's agent-browser CLI to provide \
                 navigation, interaction, screenshots, session management, and more. \
                 Requires agent-browser to be installed: cargo install agent-browser && agent-browser install",
            )
    }
}
