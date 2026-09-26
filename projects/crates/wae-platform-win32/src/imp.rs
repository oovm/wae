use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::mpsc;
use std::sync::Mutex;

use wae_platform::{DesktopIpcHandler, DesktopIpcOutcome, DesktopOpenOptions, PlatformError, Result, WindowCommand};
use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller,
    ICoreWebView2Environment,
};
use webview2_com::{
    CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler,
    WebMessageReceivedEventHandler, wait_with_pump,
};
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{E_POINTER, E_UNEXPECTED, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect, GetMessageW,
    GetWindowLongPtrW, GetWindowRect, IsZoomed, LoadCursorW, PostMessageW, PostQuitMessage,
    RegisterClassW, SetWindowLongPtrW, SetWindowPos, ShowWindow, TranslateMessage, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, MSG, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE,
    SW_SHOW, WINDOW_EX_STYLE, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_SIZE, WNDCLASSW,
    WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
};

type EventRegistrationToken = i64;

const WM_WAE_IPC: u32 = windows::Win32::UI::WindowsAndMessaging::WM_APP + 1;

const TITLEBAR_BOOT: &str = r#"
(function () {
  if (window.__waeTitlebarBoot) return;
  window.__waeTitlebarBoot = true;
  var style = document.createElement('style');
  style.textContent = [
    '[data-wae-drag-region]{app-region:drag;-webkit-app-region:drag;}',
    '[data-wae-drag-region] .no-drag,.no-drag{-webkit-app-region:no-drag;app-region:no-drag;}'
  ].join('');
  (document.head || document.documentElement).appendChild(style);

  function post(msg) {
    try {
      if (window.chrome && window.chrome.webview && window.chrome.webview.postMessage)
        window.chrome.webview.postMessage(msg);
      else if (window.ipc && window.ipc.postMessage) window.ipc.postMessage(msg);
    } catch (e) {}
  }

  var dragging = false;
  document.addEventListener('mousedown', function (e) {
    if (e.button !== 0) return;
    var t = e.target;
    if (!t || !t.closest) return;
    if (t.closest('.no-drag')) return;
    var region = t.closest('[data-wae-drag-region]');
    if (!region) return;
    if (e.detail === 2) {
      post('maximize');
      return;
    }
    dragging = true;
    post('drag_start:' + e.screenX + ',' + e.screenY);
  }, true);
  document.addEventListener('mousemove', function (e) {
    if (!dragging) return;
    post('drag_move:' + e.screenX + ',' + e.screenY);
  }, true);
  document.addEventListener('mouseup', function () {
    if (!dragging) return;
    dragging = false;
    post('drag_end');
  }, true);
  document.addEventListener('mouseleave', function () {
    if (!dragging) return;
    dragging = false;
    post('drag_end');
  }, true);
})();
"#;

#[derive(Default)]
struct DragState {
    active: bool,
    origin_mouse: (i32, i32),
    origin_window: (i32, i32),
}

struct DesktopState {
    options: DesktopOpenOptions,
    hwnd: HWND,
    controller: Option<ICoreWebView2Controller>,
    webview: Option<ICoreWebView2>,
    drag: DragState,
    handler: Box<dyn DesktopIpcHandler>,
    pending_ipc: Mutex<Option<String>>,
}

impl DesktopState {
    fn apply_window_command(&mut self, cmd: WindowCommand) {
        unsafe {
            match cmd {
                WindowCommand::Close => {
                    let _ = DestroyWindow(self.hwnd);
                }
                WindowCommand::Minimize => {
                    let _ = ShowWindow(self.hwnd, SW_MINIMIZE);
                }
                WindowCommand::Maximize => {
                    if IsZoomed(self.hwnd).as_bool() {
                        let _ = ShowWindow(self.hwnd, SW_RESTORE);
                    } else {
                        let _ = ShowWindow(self.hwnd, SW_MAXIMIZE);
                    }
                }
                WindowCommand::DragStart { screen_x, screen_y } => {
                    let mut rect = windows::Win32::Foundation::RECT::default();
                    let _ = GetWindowRect(self.hwnd, &mut rect);
                    self.drag = DragState {
                        active: true,
                        origin_mouse: (screen_x, screen_y),
                        origin_window: (rect.left, rect.top),
                    };
                }
                WindowCommand::DragMove { screen_x, screen_y } => {
                    if !self.drag.active {
                        return;
                    }
                    let nx = self.drag.origin_window.0 + (screen_x - self.drag.origin_mouse.0);
                    let ny = self.drag.origin_window.1 + (screen_y - self.drag.origin_mouse.1);
                    let _ = SetWindowPos(
                        self.hwnd,
                        None,
                        nx,
                        ny,
                        0,
                        0,
                        windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE
                            | windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER
                            | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
                    );
                }
                WindowCommand::DragEnd => {
                    self.drag.active = false;
                }
            }
        }
    }

    fn dispatch_ipc(&mut self) {
        let message = {
            let mut guard = self.pending_ipc.lock().expect("ipc lock");
            guard.take()
        };
        let Some(message) = message else {
            return;
        };

        let outcome = self.handler.on_ipc(&message);
        match outcome {
            DesktopIpcOutcome::Window(cmd) => self.apply_window_command(cmd),
            DesktopIpcOutcome::ReplyToPage(json) => {
                if let Some(webview) = self.webview.as_ref() {
                    let wide = encode_wide(&json);
                    unsafe {
                        let _ = webview.PostWebMessageAsString(PCWSTR(wide.as_ptr()));
                    }
                }
            }
            DesktopIpcOutcome::Ignored => {}
        }
    }

    fn resize_webview(&self) {
        let Some(controller) = self.controller.as_ref() else {
            return;
        };
        unsafe {
            let mut rect = windows::Win32::Foundation::RECT::default();
            if GetClientRect(self.hwnd, &mut rect).is_err() {
                return;
            }
            let _ = controller.SetBounds(rect);
        }
    }
}

fn encode_wide(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        if msg == WM_CREATE {
            let create = lparam.0 as *const windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
            let state = (*create).lpCreateParams as *mut DesktopState;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as isize);
            (*state).hwnd = hwnd;
            return LRESULT(0);
        }

        let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut DesktopState;
        if state_ptr.is_null() {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        let state = &mut *state_ptr;

        match msg {
            WM_SIZE => {
                state.resize_webview();
                LRESULT(0)
            }
            WM_WAE_IPC => {
                state.dispatch_ipc();
                LRESULT(0)
            }
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn create_environment() -> Result<ICoreWebView2Environment> {
    let (tx, rx) = mpsc::channel();
    unsafe {
        CreateCoreWebView2EnvironmentWithOptions(
            PCWSTR::null(),
            PCWSTR::null(),
            None,
            &CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(
                move |error_code, environment| {
                    error_code?;
                    tx.send(
                        environment.ok_or_else(|| windows::core::Error::from(E_POINTER)),
                    )
                    .map_err(|_| windows::core::Error::from(E_UNEXPECTED))
                },
            )),
        )
        .map_err(|e| PlatformError::Message(e.to_string()))?;
    }
    wait_with_pump(rx)
        .map_err(|e| PlatformError::Message(format!("{e:?}")))?
        .map_err(|e| PlatformError::Message(e.to_string()))
}

fn create_controller(
    hwnd: HWND,
    environment: &ICoreWebView2Environment,
) -> Result<ICoreWebView2Controller> {
    let (tx, rx) = mpsc::channel();
    let handler = CreateCoreWebView2ControllerCompletedHandler::create(Box::new(
        move |error_code, controller| {
            error_code?;
            tx.send(controller.ok_or_else(|| windows::core::Error::from(E_POINTER)))
                .map_err(|_| windows::core::Error::from(E_UNEXPECTED))
        },
    ));
    unsafe {
        environment
            .CreateCoreWebView2Controller(hwnd, &handler)
            .map_err(|e| PlatformError::Message(e.to_string()))?;
    }
    wait_with_pump(rx)
        .map_err(|e| PlatformError::Message(format!("{e:?}")))?
        .map_err(|e| PlatformError::Message(e.to_string()))
}

fn attach_webview(state: *mut DesktopState, controller: ICoreWebView2Controller) -> Result<()> {
    let webview = unsafe {
        controller
            .CoreWebView2()
            .map_err(|e| PlatformError::Message(e.to_string()))?
    };

    let state_addr = state as usize;
    let mut token = EventRegistrationToken::default();
    let message_handler = WebMessageReceivedEventHandler::create(Box::new(
        move |_, args| {
            let Some(args) = args else {
                return Ok(());
            };
            let mut message = windows::core::PWSTR::null();
            unsafe {
                args.TryGetWebMessageAsString(&mut message)?;
            }
            let text = unsafe { message.to_string().unwrap_or_default() };
            let desktop = unsafe { &*(state_addr as *mut DesktopState) };
            {
                let mut guard = desktop.pending_ipc.lock().expect("ipc lock");
                *guard = Some(text);
            }
            unsafe {
                let _ = PostMessageW(Some(desktop.hwnd), WM_WAE_IPC, WPARAM(0), LPARAM(0));
            }
            Ok(())
        },
    ));

    unsafe {
        webview
            .add_WebMessageReceived(&message_handler, &mut token)
            .map_err(|e| PlatformError::Message(e.to_string()))?;
    }

    let desktop = unsafe { &mut *state };
    desktop.controller = Some(controller);
    desktop.webview = Some(webview.clone());
    desktop.resize_webview();

    if desktop.options.undecorated {
        let script = encode_wide(TITLEBAR_BOOT);
        unsafe {
            let _ = webview.AddScriptToExecuteOnDocumentCreated(PCWSTR(script.as_ptr()), None);
        }
    }

    let wide_url = encode_wide(&desktop.options.url.clone());
    unsafe {
        webview
            .Navigate(PCWSTR(wide_url.as_ptr()))
            .map_err(|e| PlatformError::Message(e.to_string()))?;
    }

    Ok(())
}

/// Run the Win32 + WebView2 desktop shell until the window closes.
pub fn run_desktop(
    options: DesktopOpenOptions,
    handler: Box<dyn DesktopIpcHandler>,
) -> Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let class_name = w!("WAE_HOST");
        let instance = GetModuleHandleW(None).map_err(|e| PlatformError::Message(e.to_string()))?;

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW).map_err(|e| PlatformError::Message(e.to_string()))?,
            hInstance: instance.into(),
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };
        RegisterClassW(&wc);

        let title = encode_wide(&options.title);
        let mut box_state = Box::new(DesktopState {
            options,
            hwnd: HWND::default(),
            controller: None,
            webview: None,
            drag: DragState::default(),
            handler,
            pending_ipc: Mutex::new(None),
        });

        let style = if box_state.options.undecorated {
            WS_POPUP | WS_VISIBLE
        } else {
            WS_OVERLAPPEDWINDOW | WS_VISIBLE
        };

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            PCWSTR(title.as_ptr()),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1280,
            840,
            None,
            None,
            Some(instance.into()),
            Some(box_state.as_mut() as *mut DesktopState as *mut _),
        )
        .map_err(|e| PlatformError::Message(format!("CreateWindowExW failed: {e}")))?;

        box_state.hwnd = hwnd;
        let _ = ShowWindow(hwnd, SW_SHOW);

        let environment = create_environment()?;
        let controller = create_controller(hwnd, &environment)?;
        attach_webview(box_state.as_mut(), controller)?;

        eprintln!(
            "[wae-platform-win32] loaded {} undecorated={} title={}",
            box_state.options.url, box_state.options.undecorated, box_state.options.title
        );

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
