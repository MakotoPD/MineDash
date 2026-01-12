use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::fs;
use anyhow::Result;
use tauri::Manager;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaInstallation {
    pub path: String,
    pub version: Option<String>,
    pub major: Option<u32>,
    pub vendor: Option<String>,
    pub arch: Option<String>,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaValidation {
    pub is_valid: bool,
    pub version: Option<String>,
    pub major: Option<u32>,
    pub vendor: Option<String>,
    pub arch: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptiumRelease {
    pub version: String,
    pub major: u32,
    pub download_url: String,
    pub filename: String,
    pub size: u64,
    pub checksum: Option<String>,
}

struct JavaVersionInfo {
    version: String,
    major: u32,
    vendor: Option<String>,
    arch: Option<String>,
}

// === Commands ===

#[tauri::command]
pub fn detect_java_installations_cmd(app_handle: tauri::AppHandle) -> Result<Vec<JavaInstallation>, String> {
    Ok(detect_installations(&app_handle))
}

#[tauri::command]
pub fn validate_java_path_cmd(path: String) -> JavaValidation {
    validate_java_path(&path)
}

#[tauri::command]
pub fn fetch_adoptium_release_cmd(major: u32) -> Result<AdoptiumRelease, String> {
    fetch_adoptium_release(major)
}

#[tauri::command]
pub async fn download_java_cmd(major: u32, install_dir: String) -> Result<String, String> {
    let handle = tauri::async_runtime::spawn_blocking(move || {
        download_and_install_java(major, install_dir)
    });
    
    handle.await.map_err(|e| e.to_string())?
}

// === Implementation ===

pub fn detect_installations(app: &tauri::AppHandle) -> Vec<JavaInstallation> {
    let mut installations = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    let candidates = collect_java_candidates(app);

    for path in candidates {
        let path_str = path.to_string_lossy().to_string();
        if seen_paths.contains(&path_str) {
            continue;
        }
        seen_paths.insert(path_str);

        if let Some(installation) = validate_and_create_installation(&path) {
            installations.push(installation);
        }
    }

    // Sort by major version (newest first), then by path
    installations.sort_by(|a, b| {
        match (b.major, a.major) {
            (Some(b_major), Some(a_major)) => b_major.cmp(&a_major),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.path.cmp(&b.path),
        }
    });

    installations
}

pub fn validate_java_path(path_str: &str) -> JavaValidation {
    let path = Path::new(path_str);

    if !path.exists() {
        return JavaValidation {
            is_valid: false,
            version: None,
            major: None,
            vendor: None,
            arch: None,
            error: Some("Path does not exist".to_string()),
        };
    }

    match get_java_version_info(path) {
        Ok(info) => JavaValidation {
            is_valid: true,
            version: Some(info.version),
            major: Some(info.major),
            vendor: info.vendor,
            arch: info.arch,
            error: None,
        },
        Err(e) => JavaValidation {
            is_valid: false,
            version: None,
            major: None,
            vendor: None,
            arch: None,
            error: Some(e),
        },
    }
}

fn validate_and_create_installation(path: &Path) -> Option<JavaInstallation> {
    match get_java_version_info(path) {
        Ok(info) => Some(JavaInstallation {
            path: path.to_string_lossy().to_string(),
            version: Some(info.version),
            major: Some(info.major),
            vendor: info.vendor,
            arch: info.arch,
            is_valid: true,
        }),
        Err(_) => None,
    }
}

fn get_java_version_info(java_path: &Path) -> Result<JavaVersionInfo, String> {
    if !java_path.is_file() {
         return Err("Not a file".to_string());
    }

    #[cfg(target_os = "windows")]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut command = Command::new(java_path);
    command.arg("-version");
    
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let output = command.output()
        .map_err(|e| format!("Failed to execute java -version: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}\n{}", stderr, stdout);

    parse_java_version_output(&combined)
}

fn parse_java_version_output(output: &str) -> Result<JavaVersionInfo, String> {
    let lines: Vec<&str> = output.lines().collect();
    let version_line = lines.first().unwrap_or(&"");

    let version = extract_version_string(version_line)
        .ok_or_else(|| "Could not parse Java version".to_string())?;

    let major = parse_major_version(&version);
    let vendor = detect_vendor(output);
    let arch = detect_architecture(output);

    Ok(JavaVersionInfo {
        version,
        major,
        vendor,
        arch,
    })
}

fn extract_version_string(line: &str) -> Option<String> {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            return Some(line[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

fn parse_major_version(version: &str) -> u32 {
    let parts: Vec<&str> = version.split('.').collect();
    if let Some(first) = parts.first() {
        if let Ok(n) = first.parse::<u32>() {
            if n == 1 && parts.len() > 1 {
                if let Ok(second) = parts[1].parse::<u32>() {
                    return second;
                }
            }
            return n;
        }
    }
    0
}

fn detect_vendor(output: &str) -> Option<String> {
    let lower = output.to_lowercase();
    if lower.contains("temurin") || lower.contains("adoptium") {
        Some("Eclipse Temurin".to_string())
    } else if lower.contains("zulu") {
        Some("Azul Zulu".to_string())
    } else if lower.contains("corretto") {
        Some("Amazon Corretto".to_string())
    } else if lower.contains("graalvm") {
        Some("GraalVM".to_string())
    } else if lower.contains("microsoft") {
        Some("Microsoft".to_string())
    } else if lower.contains("openjdk") {
        Some("OpenJDK".to_string())
    } else if lower.contains("oracle") || lower.contains("java(tm)") {
        Some("Oracle".to_string())
    } else {
        None
    }
}

fn detect_architecture(output: &str) -> Option<String> {
    let lower = output.to_lowercase();
    if lower.contains("aarch64") || lower.contains("arm64") {
        Some("aarch64".to_string())
    } else if lower.contains("x86_64") || lower.contains("amd64") {
        Some("x86_64".to_string())
    } else if lower.contains("x86") || lower.contains("i386") || lower.contains("i686") {
        Some("x86".to_string())
    } else {
        None
    }
}

fn collect_java_candidates(app: &tauri::AppHandle) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // 1. Check VoidLink's own managed Java versions
    collect_voidlink_candidates(app, &mut candidates);

    // 2. Check JAVA_HOME
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let java_bin = Path::new(&java_home).join("bin").join(java_executable_name());
        candidates.push(java_bin);
    }

    #[cfg(target_os = "windows")]
    collect_windows_candidates(&mut candidates);

    #[cfg(not(target_os = "windows"))]
    collect_unix_candidates(&mut candidates);

    candidates
}

fn collect_voidlink_candidates(app: &tauri::AppHandle, candidates: &mut Vec<PathBuf>) {
    if let Ok(app_data) = app.path().app_data_dir() {
        let java_root = app_data.join("java");
        if java_root.exists() {
             if let Ok(entries) = std::fs::read_dir(&java_root) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        // Scan inside version dirs (e.g. java/17/jdk-17.../bin/java)
                        if let Some(bin) = find_java_executable(&entry.path()) {
                            candidates.push(bin);
                        }
                    }
                }
             }
        }
    }
}

fn java_executable_name() -> &'static str {
    #[cfg(target_os = "windows")]
    { "java.exe" }
    #[cfg(not(target_os = "windows"))]
    { "java" }
}

#[cfg(target_os = "windows")]
fn collect_windows_candidates(candidates: &mut Vec<PathBuf>) {
    let program_files = vec![
        std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string()),
        std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string()),
    ];

    let java_dirs = vec![
        "Java", "Eclipse Adoptium", "AdoptOpenJDK", "Microsoft", 
        "Zulu", "Amazon Corretto", "BellSoft",
    ];

    for pf in &program_files {
        for java_dir in &java_dirs {
            let base = Path::new(pf).join(java_dir);
            if base.exists() {
                if let Ok(entries) = std::fs::read_dir(&base) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let java_path = entry.path().join("bin").join("java.exe");
                            if java_path.exists() {
                                candidates.push(java_path);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn collect_unix_candidates(candidates: &mut Vec<PathBuf>) {
    candidates.push(PathBuf::from("/usr/bin/java"));
    let jvm_dirs = vec!["/usr/lib/jvm", "/usr/lib64/jvm", "/usr/java"];
    for jvm_dir in jvm_dirs {
        let dir = Path::new(jvm_dir);
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let java_path = entry.path().join("bin").join("java");
                     candidates.push(java_path);
                }
            }
        }
    }
    let jvm_dir = Path::new("/Library/Java/JavaVirtualMachines");
    if jvm_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(jvm_dir) {
            for entry in entries.flatten() {
                let java_path = entry.path().join("Contents").join("Home").join("bin").join("java");
                candidates.push(java_path);
            }
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let sdkman_dir = Path::new(&home).join(".sdkman").join("candidates").join("java");
        if sdkman_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&sdkman_dir) {
                 for entry in entries.flatten() {
                     if entry.path().is_dir() {
                         let java_path = entry.path().join("bin").join("java");
                         candidates.push(java_path);
                     }
                 }
            }
        }
    }
}

// === Download Logic ===

fn get_adoptium_os() -> &'static str {
    #[cfg(target_os = "windows")] { "windows" }
    #[cfg(target_os = "macos")] { "mac" }
    #[cfg(target_os = "linux")] { "linux" }
}

fn get_adoptium_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")] { "x64" }
    #[cfg(target_arch = "aarch64")] { "aarch64" }
    #[cfg(target_arch = "x86")] { "x32" }
}

pub fn fetch_adoptium_release(major: u32) -> Result<AdoptiumRelease, String> {
    let os = get_adoptium_os();
    let arch = get_adoptium_arch();
    
    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jdk&os={}&vendor=eclipse",
        major, arch, os
    );

    let client = Client::builder().user_agent("VoidLink").build().map_err(|e| e.to_string())?;
    let resp = client.get(&url).send().map_err(|e| format!("Request failed: {}", e))?;
        
    if !resp.status().is_success() {
        return Err(format!("API Error: {}", resp.status()));
    }

    let releases: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    
    let release = releases.as_array()
        .and_then(|a| a.first())
        .ok_or("No releases found".to_string())
        .map_err(|e| e)?;
        
    let binary = release.get("binary").ok_or("No binary info".to_string()).map_err(|e| e)?;
    let package = binary.get("package").ok_or("No package info".to_string()).map_err(|e| e)?;
    let version_data = release.get("version").ok_or("No version info".to_string()).map_err(|e| e)?;
    
    let semver = version_data.get("semver").and_then(|v| v.as_str()).unwrap_or("unknown");
    let download_url = package.get("link").and_then(|v| v.as_str()).ok_or("No download link".to_string()).map_err(|e| e)?;
    let filename = package.get("name").and_then(|v| v.as_str()).ok_or("No filename".to_string()).map_err(|e| e)?;
    let size = package.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
    let checksum = package.get("checksum").and_then(|v| v.as_str()).map(|s| s.to_string());

    Ok(AdoptiumRelease {
        version: semver.to_string(),
        major,
        download_url: download_url.to_string(),
        filename: filename.to_string(),
        size,
        checksum,
    })
}

pub fn download_and_install_java(major: u32, install_dir_str: String) -> Result<String, String> {
    let release = fetch_adoptium_release(major)?;
    let install_dir = PathBuf::from(&install_dir_str);
    
    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    let archive_path = install_dir.join(&release.filename);
    
    let client = Client::builder().user_agent("VoidLink").build().map_err(|e| e.to_string())?;
    let mut resp = client.get(&release.download_url).send().map_err(|e| e.to_string())?;
    
    let mut file = fs::File::create(&archive_path).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp, &mut file).map_err(|e| e.to_string())?;
    
    let extracted_dir = extract_archive(&archive_path, &install_dir).map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&archive_path);
    
    let bin = find_java_executable(&extracted_dir).ok_or("Could not find java executable in extracted files".to_string()).map_err(|e| e)?;
    
    Ok(bin.to_string_lossy().to_string())
}

fn extract_archive(archive: &Path, dest: &Path) -> Result<PathBuf> {
    let file = fs::File::open(archive).map_err(|e| anyhow::anyhow!(e))?;
    let extension = archive.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    if extension == "zip" {
        let mut zip = zip::ZipArchive::new(file).map_err(|e| anyhow::anyhow!(e))?;
        let root = zip.by_index(0).map_err(|e| anyhow::anyhow!(e))?
            .name().split('/').next().unwrap_or("").to_string();
        
        zip.extract(dest).map_err(|e| anyhow::anyhow!(e))?;
        Ok(dest.join(root))
    } else if extension == "gz" || extension == "tgz" {
        // Handle .tar.gz
        // First pass: peek at the first entry to get the root directory name
        let file_peek = fs::File::open(archive).map_err(|e| anyhow::anyhow!(e))?;
        let tar_peek = GzDecoder::new(file_peek);
        let mut archive_peek = Archive::new(tar_peek);
        
        // Use explicit loop/match to avoid complex closure type inference issues
        let mut root_name = String::new();
        if let Ok(entries) = archive_peek.entries() {
            for entry in entries {
                if let Ok(e) = entry {
                    if let Ok(path) = e.path() {
                        if let Some(first_component) = path.into_owned().to_string_lossy().split('/').next() {
                             root_name = first_component.to_string();
                             break;
                        }
                    }
                }
            }
        }

        // Second pass: actual extraction
        let file = fs::File::open(archive).map_err(|e| anyhow::anyhow!(e))?;
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        
        archive.unpack(dest).map_err(|e| anyhow::anyhow!(e))?;
        
        Ok(dest.join(root_name))
    } else {
        Err(anyhow::anyhow!("Unsupported archive format: {}", extension))
    }
}

fn find_java_executable(dir: &Path) -> Option<PathBuf> {
    let bin_java = dir.join("bin").join(java_executable_name());
    if bin_java.exists() {
        return Some(bin_java);
    }
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let deep_bin = entry.path().join("bin").join(java_executable_name());
                if deep_bin.exists() {
                    return Some(deep_bin);
                }
            }
        }
    }
    None
}
