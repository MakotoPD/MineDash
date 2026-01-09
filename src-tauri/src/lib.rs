mod frp;
mod rcon;
mod utils;
mod logs;
mod java;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use sysinfo::{Pid, ProcessesToUpdate, System};
use local_ip_address::local_ip;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Emitter,
};

// ... (existing helper functions unchanged if not shown here, assuming replace_file_content target context works)

// Skip to invoke_handler
// But I need to be careful with replace_file_content. It replaces a contiguous block. 
// I will do two edits or one large edit.
// Let's use multi_replace for safety if I need to touch top and bottom.
// Oh wait, replace_file_content replaces ONE block. 
// I should use multi_replace_file_content to add `mod logs;` at top and commands at bottom.

// Store app handle globally for menu updates
static APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);

#[derive(Serialize)]
pub struct ProcessInfo {
    pub memory_bytes: u64,
    pub cpu_usage: f32,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub cpu_count: usize,
}

#[derive(Deserialize, Clone)]
pub struct TrayServer {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[tauri::command]
fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let target_pid = Pid::from_u32(pid);
    
    if sys.process(target_pid).is_none() {
        return None;
    }

    fn sum_stats(sys: &System, pid: Pid) -> (u64, f32) {
        let mut mem = 0;
        let mut cpu = 0.0;
        
        if let Some(proc) = sys.process(pid) {
            mem += proc.memory();
            cpu += proc.cpu_usage();
        }
        
        for (child_pid, child_proc) in sys.processes() {
            if let Some(parent) = child_proc.parent() {
                if parent == pid {
                    let (c_mem, c_cpu) = sum_stats(sys, *child_pid);
                    mem += c_mem;
                    cpu += c_cpu;
                }
            }
        }
        (mem, cpu)
    }

    let (total_mem, total_cpu) = sum_stats(&sys, target_pid);

    Some(ProcessInfo {
        memory_bytes: total_mem,
        cpu_usage: total_cpu,
    })
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    SystemInfo {
        total_memory_bytes: sys.total_memory(),
        used_memory_bytes: sys.used_memory(),
        cpu_count: sys.cpus().len(),
    }
}

#[tauri::command]
fn get_local_ip() -> String {
    match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    }
}

#[tauri::command]
fn update_tray_servers(servers: Vec<TrayServer>) -> Result<(), String> {
    let handle_guard = APP_HANDLE.lock().map_err(|e| e.to_string())?;
    let app = handle_guard.as_ref().ok_or("App handle not initialized")?;
    
    let mut menu_items: Vec<MenuItem<tauri::Wry>> = vec![];
    
    if let Ok(item) = MenuItem::new(app, "VoidLink", false, None::<&str>) {
        menu_items.push(item);
    }
    
    if let Ok(item) = MenuItem::new(app, "─────────────", false, None::<&str>) {
        menu_items.push(item);
    }
    
    for server in &servers {
        let status_icon = if server.status == "online" { "▶" } else { "⏹" };
        let label = format!("{} {}", status_icon, server.name);
        if let Ok(item) = MenuItem::with_id(app, format!("server_{}", server.id), label, true, None::<&str>) {
            menu_items.push(item);
        }
    }
    
    if let Ok(item) = MenuItem::new(app, "─────────────", false, None::<&str>) {
        menu_items.push(item);
    }
    
    if let Ok(item) = MenuItem::with_id(app, "show", "Show VoidLink", true, None::<&str>) {
        menu_items.push(item);
    }
    if let Ok(item) = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>) {
        menu_items.push(item);
    }
    
    let menu = Menu::with_items(app, &menu_items.iter().map(|i| i as &dyn tauri::menu::IsMenuItem<tauri::Wry>).collect::<Vec<_>>())
        .map_err(|e| e.to_string())?;
    
    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_menu(Some(menu));
    }
    
    Ok(())
}

#[tauri::command]
fn kill_process(pid: u32) -> bool {
    #[cfg(windows)]
    {
        // Use /T to kill the entire process tree (important for Java which spawns child processes)
        let result = std::process::Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .output();
        
        match result {
            Ok(output) => {
                log::info!("Killed process tree for PID {}: {:?}", pid, output.status.success());
                output.status.success()
            }
            Err(e) => {
                log::error!("Failed to kill process {}: {}", pid, e);
                false
            }
        }
    }
    #[cfg(not(windows))]
    {
        let result = std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output();
        result.map(|o| o.status.success()).unwrap_or(false)
    }
}

#[tauri::command]
fn quit_app() {
    frp::stop_all_tunnels();
    std::process::exit(0);
}

#[tauri::command]
fn cleanup_and_quit(app: tauri::AppHandle) {
    // Stop all FRP tunnels
    frp::stop_all_tunnels();
    // Exit the app
    app.exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Ok(mut handle) = APP_HANDLE.lock() {
                *handle = Some(app.handle().clone());
            }
            
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            
            let show = MenuItem::with_id(app, "show", "Show VoidLink", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("VoidLink")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app: &tauri::AppHandle, event| {
                    let id = event.id.as_ref();
                    
                    if id == "quit" {
                        frp::stop_all_tunnels();
                        app.exit(0);
                    } else if id == "show" {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    } else if id.starts_with("server_") {
                        let server_id = id.replace("server_", "");
                        let _ = app.emit("tray-open-server", server_id);
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
          // Check for deep link in args
          for arg in argv {
              if arg.starts_with("voidlink://") {
                  let _ = app.emit("deep-link://new-url", vec![arg]);
              }
          }
          
          if let Some(window) = app.get_webview_window("main") {
              let _ = window.set_focus();
              let _ = window.show();
              let _ = window.unminimize();
          }
        }))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_process_info,
            get_system_info,
            get_local_ip,
            update_tray_servers,
            kill_process,
            quit_app,
            cleanup_and_quit,
            // FRP Tunnel commands
            frp::frp_start_tunnel,
            frp::frp_stop_tunnel,
            frp::frp_get_status,
            // RCON commands
            rcon::rcon_send_command,
            rcon::rcon_stop_server,
            utils::read_log_tail,
            // Java commands
            java::detect_java_installations_cmd,
            java::validate_java_path_cmd,
            java::fetch_adoptium_release_cmd,
            java::download_java_cmd,
            // Logs commands
            logs::list_crash_reports_cmd,
            logs::read_crash_report_cmd,
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Cleanup all FRP tunnels when window closes
                frp::stop_all_tunnels();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
