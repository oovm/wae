//! 桌面壳：无边框自绘标题栏（`WAE_UNDECORATED=1`）+ IPC 窗控。
//!
//! 拖拽：WebView2 上异步 IPC 再调 `drag_window()` 经常失效（按下态已丢）。
//! 因此用 `drag_start` / `drag_move` / `drag_end` + `set_outer_position` 手动跟手。

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use tao::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Debug)]
enum UserEvent {
    Close,
    Minimize,
    Maximize,
    DragStart { screen_x: i32, screen_y: i32 },
    DragMove { screen_x: i32, screen_y: i32 },
    DragEnd,
}

#[derive(Default)]
struct DragState {
    active: bool,
    origin_mouse: (i32, i32),
    origin_window: (i32, i32),
}

fn main() {
    let url = parse_url().unwrap_or_else(|| {
        eprintln!("用法: wae-desktop --url <http://127.0.0.1:5173/>");
        std::process::exit(2);
    });
    let title = env::var("WAE_WINDOW_TITLE").unwrap_or_else(|_| "WAE".to_string());
    let undecorated = env_flag("WAE_UNDECORATED");

    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let mut builder = WindowBuilder::new()
        .with_title(title.clone())
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 840.0));
    if undecorated {
        builder = builder.with_decorations(false);
    }

    let window = builder.build(&event_loop).expect("创建窗口失败");
    let window = Rc::new(window);
    let proxy_for_ipc = proxy.clone();
    let drag = Rc::new(RefCell::new(DragState::default()));

    // 注入：标题栏区域用 screen 坐标跟手拖动（不依赖 drag_window 时机）。
    let boot_script = r#"
(function () {
  if (window.__waeTitlebarBoot) return;
  window.__waeTitlebarBoot = true;
  var style = document.createElement('style');
  style.textContent = [
    '[data-wry-drag-region]{app-region:drag;-webkit-app-region:drag;}',
    '[data-wry-drag-region] .no-drag,.no-drag{-webkit-app-region:no-drag;app-region:no-drag;}'
  ].join('');
  (document.head || document.documentElement).appendChild(style);

  function post(msg) {
    try {
      if (window.ipc && window.ipc.postMessage) window.ipc.postMessage(msg);
      else if (window.chrome && window.chrome.webview && window.chrome.webview.postMessage)
        window.chrome.webview.postMessage(msg);
    } catch (e) {}
  }

  var dragging = false;
  document.addEventListener('mousedown', function (e) {
    if (e.button !== 0) return;
    var t = e.target;
    if (!t || !t.closest) return;
    if (t.closest('.no-drag')) return;
    var region = t.closest('[data-wry-drag-region]');
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

    let mut webview_builder = WebViewBuilder::new()
        .with_url(&url)
        .with_ipc_handler(move |req| {
            let body = req.body().as_str();
            let ev = parse_ipc(body);
            if let Some(ev) = ev {
                let _ = proxy_for_ipc.send_event(ev);
            }
        })
        .with_accept_first_mouse(true);

    if undecorated {
        webview_builder = webview_builder.with_initialization_script(boot_script);
    }

    let _webview = webview_builder
        .build(&*window)
        .expect("创建 WebView 失败（Windows 需已安装 WebView2 Runtime）");

    eprintln!("[wae-desktop] loaded {url} undecorated={undecorated} title={title}");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Close) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Minimize) => {
                window.set_minimized(true);
            }
            Event::UserEvent(UserEvent::Maximize) => {
                window.set_maximized(!window.is_maximized());
            }
            Event::UserEvent(UserEvent::DragStart { screen_x, screen_y }) => {
                let pos = window
                    .outer_position()
                    .unwrap_or_else(|_| PhysicalPosition::new(0, 0));
                *drag.borrow_mut() = DragState {
                    active: true,
                    origin_mouse: (screen_x, screen_y),
                    origin_window: (pos.x, pos.y),
                };
            }
            Event::UserEvent(UserEvent::DragMove { screen_x, screen_y }) => {
                let st = drag.borrow();
                if !st.active {
                    return;
                }
                let nx = st.origin_window.0 + (screen_x - st.origin_mouse.0);
                let ny = st.origin_window.1 + (screen_y - st.origin_mouse.1);
                drop(st);
                window.set_outer_position(PhysicalPosition::new(nx, ny));
            }
            Event::UserEvent(UserEvent::DragEnd) => {
                drag.borrow_mut().active = false;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn parse_ipc(body: &str) -> Option<UserEvent> {
    match body {
        "window:minimize" | "minimize" => Some(UserEvent::Minimize),
        "window:maximize" | "maximize" => Some(UserEvent::Maximize),
        "window:close" | "close" => Some(UserEvent::Close),
        "drag_end" | "window:drag-end" => Some(UserEvent::DragEnd),
        _ => {
            if let Some(rest) = body.strip_prefix("drag_start:") {
                let (x, y) = parse_xy(rest)?;
                return Some(UserEvent::DragStart {
                    screen_x: x,
                    screen_y: y,
                });
            }
            if let Some(rest) = body.strip_prefix("drag_move:") {
                let (x, y) = parse_xy(rest)?;
                return Some(UserEvent::DragMove {
                    screen_x: x,
                    screen_y: y,
                });
            }
            // 兼容旧消息：尽量用 drag_window（可能仍失效）
            if body == "drag_window" || body == "window:drag" {
                // 无坐标时无法手动拖；忽略
                return None;
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

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes")
    )
}

fn parse_url() -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--url" {
            return args.next();
        }
        if let Some(rest) = a.strip_prefix("--url=") {
            return Some(rest.to_string());
        }
    }
    None
}
