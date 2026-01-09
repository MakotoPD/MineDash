// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {

  #[cfg(target_os = "linux")]
    {
        // Only set if not already configured by the user
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }

        // On some Wayland setups the AppImage fails to create an EGL display.
        // Fallback to X11 when running from AppImage unless the user overrides.
        let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
            || matches!(std::env::var("XDG_SESSION_TYPE"), Ok(session) if session == "wayland");
        let is_appimage = std::env::var("APPIMAGE").is_ok();
        if is_appimage && is_wayland && std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
    }
  app_lib::run();
}
