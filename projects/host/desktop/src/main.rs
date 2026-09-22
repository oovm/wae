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
    // WebView 与窗口同寿；IPC 需持有窗口引用。
    let window = std::rc::Rc::new(window);
    let window_for_ipc = window.clone();
    let proxy_for_ipc = proxy.clone();

    let _webview = WebViewBuilder::new()
        .with_url(&url)
        .with_ipc_handler(move |req| {
            let body = req.body();
            match body.as_str() {
                "window:minimize" => window_for_ipc.set_minimized(true),
                "window:maximize" => {
                    window_for_ipc.set_maximized(!window_for_ipc.is_maximized());
                }
                "window:close" => {
                    let _ = proxy_for_ipc.send_event(UserEvent::Close);
                }
                // 自绘标题栏拖拽：须在鼠标左键按下路径上同步触发。
                "window:drag" => {
                    let _ = window_for_ipc.drag_window();
                }
                _ => {}
            }
        })
        .build(&*window)
        .expect("创建 WebView 失败（Windows 需已安装 WebView2 Runtime）");

    eprintln!(
        "[wae-desktop] loaded {url} undecorated={undecorated} title={title}"
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Close) => {
                *control_flow = ControlFlow::Exit;
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
