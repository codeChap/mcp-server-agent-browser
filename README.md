# mcp-server-agent-browser

An MCP (Model Context Protocol) server that exposes browser automation tools by wrapping [Vercel's agent-browser](https://github.com/ArcadeAI/agent-browser) CLI. Communicates over stdio using JSON-RPC 2.0.

## Prerequisites

Install the `agent-browser` CLI:

```bash
cargo install agent-browser
agent-browser install
```

## Build

```bash
cargo build --release
```

The binary is output to `target/release/mcp-server-agent-browser`.

## Usage

Run as a stdio MCP server:

```bash
./target/release/mcp-server-agent-browser
```

Or with debug logging:

```bash
RUST_LOG=debug cargo run
```

### Claude Desktop / Claude Code

Add to your MCP server configuration:

```json
{
  "mcpServers": {
    "agent-browser": {
      "command": "/path/to/mcp-server-agent-browser"
    }
  }
}
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `AGENT_BROWSER_PATH` | Path to the `agent-browser` binary | `agent-browser` (found via `$PATH`) |

## Tools

### Navigation

| Tool | Description |
|------|-------------|
| `browser_navigate` | Navigate to a URL |
| `browser_go_back` | Navigate back in history |
| `browser_go_forward` | Navigate forward in history |
| `browser_reload` | Reload the current page |

### Interaction

| Tool | Description |
|------|-------------|
| `browser_click` | Click an element by selector or `@ref` |
| `browser_dblclick` | Double-click an element |
| `browser_fill` | Fill a text input (clears existing value) |
| `browser_type` | Type text character by character |
| `browser_keyboard_type` | Type with real keystrokes (no selector) |
| `browser_keyboard_inserttext` | Insert text without key events |
| `browser_press` | Press a keyboard key |
| `browser_hover` | Hover over an element |
| `browser_focus` | Focus an element |
| `browser_scroll` | Scroll the page (up/down/left/right) |
| `browser_scroll_into_view` | Scroll an element into view |
| `browser_select` | Select a dropdown option |
| `browser_check` | Check a checkbox or radio button |
| `browser_uncheck` | Uncheck a checkbox |
| `browser_drag` | Drag an element to another |
| `browser_upload` | Upload files to a file input |
| `browser_download` | Download a file by clicking an element |

### Information

| Tool | Description |
|------|-------------|
| `browser_get_text` | Get text content from an element or page |
| `browser_get_html` | Get HTML content (inner or outer) |
| `browser_get_attribute` | Get an attribute value |
| `browser_get_url` | Get the current page URL |
| `browser_get_title` | Get the current page title |
| `browser_snapshot` | Get an accessibility tree snapshot with `@ref` identifiers |

### Element State

| Tool | Description |
|------|-------------|
| `browser_is_visible` | Check if an element is visible |
| `browser_is_enabled` | Check if an element is enabled |
| `browser_is_checked` | Check if a checkbox/radio is checked |

### Capture

| Tool | Description |
|------|-------------|
| `browser_screenshot` | Screenshot the page or an element |
| `browser_pdf` | Generate a PDF of the page |

### Sessions

| Tool | Description |
|------|-------------|
| `browser_new_session` | Create a new isolated browser session |
| `browser_close_session` | Close a specific session |
| `browser_close` | Close the entire browser daemon |

### Wait

| Tool | Description |
|------|-------------|
| `browser_wait` | Wait for an element or a delay (ms) |

### Cookies

| Tool | Description |
|------|-------------|
| `browser_get_cookies` | Get cookies |
| `browser_set_cookies` | Set cookies |
| `browser_clear_cookies` | Clear all cookies |

### Other

| Tool | Description |
|------|-------------|
| `browser_evaluate` | Execute JavaScript and return the result |
| `browser_get_console` | Get console log messages |
| `browser_get_network` | Get captured network requests |
| `browser_connect` | Connect to a browser via Chrome DevTools Protocol |

## Session Management

All interaction tools accept an optional `session_id` parameter. Use `browser_new_session` to create isolated sessions with independent cookies, storage, and viewport, then pass the returned ID to subsequent calls.

## License

MIT
