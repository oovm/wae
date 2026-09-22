//! 最小桌面壳：打开原生 WebView 窗口并加载给定 URL（开发期通常是 Vite）。

use std::env;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() {
    let url = parse_url().unwrap_or_else(|| {
        eprintln!("用法: wae-desktop --url <http://127.0.0.1:5173/>");
        std::process::exit(2);
    });
    let title = env::var("WAE_WINDOW_TITLE").unwrap_or_else(|_| "WAE".to_string());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(tao::dpi::LogicalSize::new(1100.0, 720.0))
        .build(&event_loop)
        .expect("创建窗口失败");

    let _webview = WebViewBuilder::new()
        .with_url(&url)
        .build(&window)
        .expect("创建 WebView 失败（Windows 需已安装 WebView2 Runtime）");

    eprintln!("[wae-desktop] loaded {url}");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
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
