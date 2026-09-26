//! Low-level platform binding traits (implemented per OS in `wae-platform-*`).

use crate::{PlatformError, Result};

/// Native top-level window surface.
pub trait PlatformWindow {
    /// Set the window title string.
    fn set_title(&self, title: &str) -> Result<()>;
    /// Minimize or restore from minimized state.
    fn set_minimized(&self, minimized: bool) -> Result<()>;
    /// Maximize or restore from maximized state.
    fn set_maximized(&self, maximized: bool) -> Result<()>;
    /// Move the window outer frame to screen coordinates.
    fn set_outer_position(&self, x: i32, y: i32) -> Result<()>;
    /// Current outer frame top-left in screen coordinates.
    fn outer_position(&self) -> Result<(i32, i32)>;
}

/// System WebView control embedded in a [`PlatformWindow`].
pub trait PlatformWebView {
    /// Navigate to an absolute http(s) or file URL.
    fn navigate(&self, url: &str) -> Result<()>;
    /// Post a JSON string to the page (`WebMessageReceived` / equivalent).
    fn post_message_to_page(&self, json: &str) -> Result<()>;
    /// Register a handler for page → host IPC payloads.
    fn set_ipc_handler(&mut self, handler: Box<dyn Fn(&str) + Send>) -> Result<()>;
    /// Inject JavaScript to run on each document creation.
    fn inject_script_on_document_created(&self, script: &str) -> Result<()>;
}

/// Blocking message loop until the shell exits.
pub trait PlatformRuntime {
    /// Run the platform event loop; normally does not return until quit.
    fn run(self: Box<Self>) -> Result<()>;
}

/// Factory for a concrete desktop shell on the current target OS.
pub trait PlatformDesktopFactory {
    /// Create window + webview for the given parent HWND-equivalent when applicable.
    fn open_desktop(
        options: &crate::DesktopOpenOptions,
        handler: Box<dyn crate::DesktopIpcHandler>,
    ) -> Result<Box<dyn PlatformRuntime>>;
}

impl PlatformError {
    /// Wrap a display message as [`PlatformError::Message`].
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}
