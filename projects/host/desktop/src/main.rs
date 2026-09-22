//! 桌面壳：打开原生 WebView。支持无边框自绘标题栏（`WAE_UNDECORATED=1`）与 IPC 窗控。

use std::env;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Debug)]
enum UserEvent {
    Close,
    DragWindow,
    Minimize,
    Maximize,
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
    let window = std::rc::Rc::new(window);
    let proxy_for_ipc = proxy.clone();

    let _webview = WebViewBuilder::new()
        .with_url(&url)
        .with_ipc_handler(move |req| {
            // 窗控必须投递到事件循环线程（wry 官方 custom_titlebar 做法）。
            let body = req.body();
            let ev = match body.as_str() {
                "window:minimize" | "minimize" => Some(UserEvent::Minimize),
                "window:maximize" | "maximize" => Some(UserEvent::Maximize),
                "window:close" | "close" => Some(UserEvent::Close),
                "window:drag" | "drag_window" => Some(UserEvent::DragWindow),
                _ => None,
            };
            if let Some(ev) = ev {
                let _ = proxy_for_ipc.send_event(ev);
            }
        })
        .build(&*window)
        .expect("创建 WebView 失败（Windows 需已安装 WebView2 Runtime）");

    eprintln!("[wae-desktop] loaded {url} undecorated={undecorated} title={title}");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Close) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::DragWindow) => {
                let _ = window.drag_window();
            }
            Event::UserEvent(UserEvent::Minimize) => {
                window.set_minimized(true);
            }
            Event::UserEvent(UserEvent::Maximize) => {
                window.set_maximized(!window.is_maximized());
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
