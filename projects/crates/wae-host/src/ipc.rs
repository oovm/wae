//! Window-control IPC parsing shared by all desktop platforms.

use wae_platform::{DesktopIpcOutcome, WindowCommand};

/// Parse legacy window-control IPC strings from the page boot script.
pub fn parse_window_ipc(body: &str) -> Option<WindowCommand> {
    match body {
        "window:minimize" | "minimize" => Some(WindowCommand::Minimize),
        "window:maximize" | "maximize" => Some(WindowCommand::Maximize),
        "window:close" | "close" => Some(WindowCommand::Close),
        "drag_end" | "window:drag-end" => Some(WindowCommand::DragEnd),
        _ => {
            if let Some(rest) = body.strip_prefix("drag_start:") {
                let (x, y) = parse_xy(rest)?;
                return Some(WindowCommand::DragStart { screen_x: x, screen_y: y });
            }
            if let Some(rest) = body.strip_prefix("drag_move:") {
                let (x, y) = parse_xy(rest)?;
                return Some(WindowCommand::DragMove { screen_x: x, screen_y: y });
            }
            None
        }
    }
}

fn parse_xy(s: &str) -> Option<(i32, i32)> {
    let mut it = s.split(',');
    let x = it.next()?.parse().ok()?;
    let y = it.next()?.parse().ok()?;
    Some((x, y))
}

/// Routes page IPC to window commands or [`wae_bridge::WebViewRuntime`].
pub struct HostIpcRouter {
    bridge: wae_bridge::WebViewRuntime,
}

impl HostIpcRouter {
    /// Create a router with a fresh bridge runtime.
    pub fn new() -> Self {
        Self { bridge: wae_bridge::WebViewRuntime::new() }
    }

    /// Handle one IPC payload from the WebView.
    pub fn handle(&mut self, message: &str) -> DesktopIpcOutcome {
        if let Some(cmd) = parse_window_ipc(message) {
            return DesktopIpcOutcome::Window(cmd);
        }

        match serde_json::from_str::<wae_types::ClientMessage>(message) {
            Ok(client) => match self.bridge.handle(client) {
                Some(host) => match serde_json::to_string(&host) {
                    Ok(json) => DesktopIpcOutcome::ReplyToPage(json),
                    Err(err) => DesktopIpcOutcome::ReplyToPage(
                        serde_json::json!({
                            "kind": "error",
                            "code": "Internal",
                            "message": err.to_string(),
                        })
                        .to_string(),
                    ),
                },
                None => DesktopIpcOutcome::Ignored,
            },
            Err(_) => DesktopIpcOutcome::Ignored,
        }
    }
}

impl Default for HostIpcRouter {
    fn default() -> Self {
        Self::new()
    }
}
