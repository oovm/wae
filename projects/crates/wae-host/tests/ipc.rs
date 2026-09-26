use wae_host::parse_window_ipc;
use wae_platform::WindowCommand;

#[test]
fn parses_window_commands() {
    assert_eq!(parse_window_ipc("minimize"), Some(WindowCommand::Minimize));
    assert_eq!(
        parse_window_ipc("drag_start:10,20"),
        Some(WindowCommand::DragStart {
            screen_x: 10,
            screen_y: 20
        })
    );
}
