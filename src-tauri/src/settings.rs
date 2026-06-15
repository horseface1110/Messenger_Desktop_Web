use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("failed to resolve app config directory")]
    ConfigDir,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl serde::Serialize for SettingsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub start_on_login: bool,
    pub close_to_tray: bool,
    pub start_minimized: bool,
    pub shortcut: String,
    pub messenger_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_on_login: false,
            close_to_tray: true,
            start_minimized: false,
            shortcut: "Ctrl+Shift+M".to_string(),
            messenger_url: "https://www.messenger.com".to_string(),
        }
    }
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, SettingsError> {
    read_settings(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), SettingsError> {
    write_settings(&app, &settings)
}

pub fn read_settings(app: &AppHandle) -> Result<Settings, SettingsError> {
    let path = settings_path(app)?;

    if !path.exists() {
        return Ok(Settings::default());
    }

    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn write_settings(app: &AppHandle, settings: &Settings) -> Result<(), SettingsError> {
    let path = settings_path(app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, serde_json::to_string_pretty(settings)?)?;
    Ok(())
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, SettingsError> {
    let config_dir = app.path().app_config_dir().map_err(|_| SettingsError::ConfigDir)?;
    Ok(config_dir.join("settings.json"))
}
