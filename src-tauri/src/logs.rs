use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReport {
    pub name: String,
    pub path: String,
    pub created: u64,
    pub content: Option<String>,
}

#[tauri::command]
pub fn list_crash_reports_cmd(server_path: String) -> Result<Vec<CrashReport>, String> {
    let crash_reports_dir = PathBuf::from(&server_path).join("crash-reports");
    
    if !crash_reports_dir.exists() {
        return Ok(Vec::new());
    }

    let mut reports = Vec::new();

    if let Ok(entries) = fs::read_dir(crash_reports_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    let created = metadata.created().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
                        
                    reports.push(CrashReport {
                        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        created,
                        content: None, // Don't load content for list
                    });
                }
            }
        }
    }
    
    // Sort by created desc
    reports.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(reports)
}

#[tauri::command]
pub fn read_crash_report_cmd(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}
