use tauri::{AppHandle, Manager, Theme};

use crate::settings::Settings;

pub fn apply_theme(app: &AppHandle, settings: &Settings) {
    let theme = match settings.theme.as_str() {
        "light" => Some(Theme::Light),
        "dark" => Some(Theme::Dark),
        _ => None,
    };

    for window in app.webview_windows().values() {
        let _ = window.set_theme(theme);
        let _ = window.eval(theme_script(&settings.theme));
    }
}

fn theme_script(theme: &str) -> String {
    format!(
        r#"
(() => {{
  const mode = {theme:?};
  document.documentElement.dataset.messengerDesktopTheme = mode;
  document.documentElement.style.colorScheme =
    mode === "light" || mode === "dark" ? mode : "";
}})();
"#
    )
}
