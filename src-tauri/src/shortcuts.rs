use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

pub fn setup(app: &mut App) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);

    app.global_shortcut()
        .on_shortcut(shortcut, |app, _shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }

            if let Some(window) = app.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })?;

    Ok(())
}
