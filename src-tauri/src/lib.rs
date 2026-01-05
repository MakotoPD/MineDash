#[cfg_attr(mobile, tauri::mobile_entry_point)]

use sysinfo::{Pid, ProcessesToUpdate, System};
use local_ip_address::local_ip;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Emitter,
};
use std::sync::Mutex;

// Store app handle globally for menu updates
static APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);

#[derive(serde::Serialize)]
pub struct ProcessInfo {
    pub memory_bytes: u64,
    pub cpu_usage: f32,
}

#[derive(serde::Serialize)]
pub struct SystemInfo {
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub cpu_count: usize,
}

#[derive(serde::Deserialize, Clone)]
pub struct TrayServer {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[tauri::command]
fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    let mut sys = System::new();
    // We must refresh all processes to find children relationships
    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let target_pid = Pid::from_u32(pid);
    
    // Check if process exists first
    if sys.process(target_pid).is_none() {
        return None;
    }

    // Since we need to traverse down, and sysinfo doesn't give direct child links easily without iteration,
    // we can iterate once to build a simple map of Parent -> Children, OR just iterate repeatedly.
    // Iterating repeatedly is O(N * Depth), which is small enough for process trees.
    
    // Recursive closure to sum stats
    fn sum_stats(sys: &System, pid: Pid) -> (u64, f32) {
        let mut mem = 0;
        let mut cpu = 0.0;
        
        // Add self stats
        if let Some(proc) = sys.process(pid) {
            mem += proc.memory();
            cpu += proc.cpu_usage();
        }
        
        // Find children (inefficient O(N) scan per node but robust)
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
    
    // Build new menu
    let mut menu_items: Vec<MenuItem<tauri::Wry>> = vec![];
    
    // Add header
    if let Ok(item) = MenuItem::new(app, "MineDash", false, None::<&str>) {
        menu_items.push(item);
    }
    
    // Add separator-like disabled item
    if let Ok(item) = MenuItem::new(app, "─────────────", false, None::<&str>) {
        menu_items.push(item);
    }
    
    // Add servers
    for server in &servers {
        let status_icon = if server.status == "online" { "🟢" } else { "🔴" };
        let label = format!("{} {}", status_icon, server.name);
        if let Ok(item) = MenuItem::with_id(app, format!("server_{}", server.id), label, true, None::<&str>) {
            menu_items.push(item);
        }
    }
    
    // Add separator
    if let Ok(item) = MenuItem::new(app, "─────────────", false, None::<&str>) {
        menu_items.push(item);
    }
    
    // Add show/quit
    if let Ok(item) = MenuItem::with_id(app, "show", "Show MineDash", true, None::<&str>) {
        menu_items.push(item);
    }
    if let Ok(item) = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>) {
        menu_items.push(item);
    }
    
    // Build menu from items
    let menu = Menu::with_items(app, &menu_items.iter().map(|i| i as &dyn tauri::menu::IsMenuItem<tauri::Wry>).collect::<Vec<_>>())
        .map_err(|e| e.to_string())?;
    
    // Update tray menu
    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_menu(Some(menu));
    }
    
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Store app handle for later use
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
            
            // Create initial tray menu
            let show = MenuItem::with_id(app, "show", "Show MineDash", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            
            // Create tray icon
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("MineDash")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app: &tauri::AppHandle, event| {
                    let id = event.id.as_ref();
                    
                    if id == "quit" {
                        app.exit(0);
                    } else if id == "show" {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    } else if id.starts_with("server_") {
                        let server_id = id.replace("server_", "");
                        // Emit event to frontend
                        let _ = app.emit("tray-open-server", server_id);
                        // Show window
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_process_info, get_system_info, get_local_ip, update_tray_servers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
