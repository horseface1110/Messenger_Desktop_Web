import { invoke } from "@tauri-apps/api/core";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import "./style.css";

type Settings = {
  start_on_login: boolean;
  close_to_tray: boolean;
  start_minimized: boolean;
  shortcut: string;
  messenger_url: string;
};

function requireElement<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector);

  if (!element) {
    throw new Error(`Missing required element: ${selector}`);
  }

  return element;
}

const form = requireElement<HTMLFormElement>("#settings-form");
const status = requireElement<HTMLElement>("#status");
const startOnLogin = requireElement<HTMLInputElement>("#start-on-login");
const closeToTray = requireElement<HTMLInputElement>("#close-to-tray");
const startMinimized = requireElement<HTMLInputElement>("#start-minimized");
const shortcut = requireElement<HTMLInputElement>("#shortcut");
const messengerUrl = requireElement<HTMLSelectElement>("#messenger-url");

function render(settings: Settings) {
  startOnLogin.checked = settings.start_on_login;
  closeToTray.checked = settings.close_to_tray;
  startMinimized.checked = settings.start_minimized;
  shortcut.value = settings.shortcut;
  messengerUrl.value = settings.messenger_url;
}

function collect(): Settings {
  return {
    start_on_login: startOnLogin.checked,
    close_to_tray: closeToTray.checked,
    start_minimized: startMinimized.checked,
    shortcut: shortcut.value,
    messenger_url: messengerUrl.value,
  };
}

async function loadSettings() {
  const settings = await invoke<Settings>("get_settings");
  settings.start_on_login = await isEnabled().catch(() => settings.start_on_login);
  render(settings);
}

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  status.textContent = "Saving...";

  const settings = collect();
  await invoke("save_settings", { settings });

  if (settings.start_on_login) {
    await enable().catch(() => undefined);
  } else {
    await disable().catch(() => undefined);
  }

  status.textContent = "Saved.";
  window.setTimeout(() => {
    status.textContent = "";
  }, 1800);
});

loadSettings().catch((error) => {
  console.error(error);
  status.textContent = "Could not load settings.";
});
