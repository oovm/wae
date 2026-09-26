//! WAE desktop dev binary — loads a frontend URL via the self-hosted platform shell.

use std::env;

use wae_host::run_desktop;
use wae_platform::DesktopOpenOptions;

fn main() {
    let url = parse_url().unwrap_or_else(|| {
        eprintln!("用法: wae-desktop --url <http://127.0.0.1:5173/>");
        std::process::exit(2);
    });
    let title = env::var("WAE_WINDOW_TITLE").unwrap_or_else(|_| "WAE".to_string());
    let undecorated = env_flag("WAE_UNDECORATED");

    let options = DesktopOpenOptions::new(url, title).undecorated(undecorated);

    if let Err(err) = run_desktop(options) {
        eprintln!("[wae-desktop] {err}");
        std::process::exit(1);
    }
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
