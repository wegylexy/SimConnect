//! `SimConnect.cfg` discovery and parsing.
//!
//! The sim (and third-party client tools) write a `SimConnect.cfg` INI file
//! with `[SimConnect]` (or `[SimConnect.N]` for additional entries)
//! sections describing how to reach the SimConnect TCP/pipe server —
//! primarily useful for remote connections, since local same-machine
//! clients normally just use the well-known named pipe
//! (`transport::DEFAULT_PIPE_NAME`).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct CfgEntry {
    pub address: String,
    pub port: u16,
}

#[cfg(windows)]
fn documents_folder() -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    #[repr(C)]
    struct KnownFolderId {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }
    // FOLDERID_Documents: {FDD39AD0-238F-46AF-ADB4-6C85480369C7}
    const FOLDERID_DOCUMENTS: KnownFolderId = KnownFolderId {
        data1: 0xFDD39AD0,
        data2: 0x238F,
        data3: 0x46AF,
        data4: [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7],
    };

    #[link(name = "shell32")]
    extern "system" {
        fn SHGetKnownFolderPath(
            rfid: *const KnownFolderId,
            dwFlags: u32,
            hToken: *mut std::ffi::c_void,
            ppszPath: *mut *mut u16,
        ) -> i32;
    }

    #[link(name = "ole32")]
    extern "system" {
        fn CoTaskMemFree(pv: *mut std::ffi::c_void);
    }

    unsafe {
        let mut path_ptr: *mut u16 = std::ptr::null_mut();
        let hr = SHGetKnownFolderPath(&FOLDERID_DOCUMENTS, 0, std::ptr::null_mut(), &mut path_ptr);
        if hr == 0 && !path_ptr.is_null() {
            let mut len = 0;
            while *path_ptr.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(path_ptr, len);
            let os_str = OsString::from_wide(slice);
            CoTaskMemFree(path_ptr as *mut std::ffi::c_void);
            return Some(PathBuf::from(os_str));
        }
    }
    if let Ok(profile) = std::env::var("USERPROFILE") {
        let p = PathBuf::from(profile).join("Documents");
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// Standard search locations, in order, for a `SimConnect.cfg`:
/// 1. Process directory (`current_exe().parent()`)
/// 2. Current working directory (`current_dir()`)
/// 3. User's Documents folder (and Prepar3D / FSX Files subdirectories)
/// 4. `%APPDATA%` (Microsoft Flight Simulator, Lockheed Martin Prepar3D, FSX)
/// 5. `%USERPROFILE%` profile root
pub fn default_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut push_unique = |p: PathBuf| {
        if !paths.contains(&p) {
            paths.push(p);
        }
    };

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            push_unique(dir.join("SimConnect.cfg"));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        push_unique(cwd.join("SimConnect.cfg"));
    }

    #[cfg(windows)]
    if let Some(docs) = documents_folder() {
        push_unique(docs.join("SimConnect.cfg"));
        for p3d_dir in &[
            "Prepar3D v6 Files",
            "Prepar3D v5 Files",
            "Prepar3D v4 Files",
            "Prepar3D v3 Files",
            "Prepar3D v2 Files",
            "Prepar3D Files",
            "Flight Simulator X Files",
        ] {
            push_unique(docs.join(p3d_dir).join("SimConnect.cfg"));
        }
    }

    if let Ok(appdata) = std::env::var("APPDATA") {
        let appdata_path = Path::new(&appdata);
        push_unique(
            appdata_path
                .join("Microsoft Flight Simulator")
                .join("SimConnect.cfg"),
        );
        for p3d_ver in &["v6", "v5", "v4", "v3", "v2"] {
            push_unique(
                appdata_path
                    .join("Lockheed Martin")
                    .join(format!("Prepar3D {p3d_ver}"))
                    .join("SimConnect.cfg"),
            );
        }
        push_unique(
            appdata_path
                .join("Microsoft")
                .join("FSX")
                .join("SimConnect.cfg"),
        );
        push_unique(
            appdata_path
                .join("Microsoft")
                .join("FSX-SE")
                .join("SimConnect.cfg"),
        );
    }

    if let Ok(profile) = std::env::var("USERPROFILE") {
        push_unique(Path::new(&profile).join("SimConnect.cfg"));
    }

    paths
}

/// Parses a `SimConnect.cfg` file's `[SimConnect]`/`[SimConnect.N]` sections.
/// Section index `0` corresponds to the bare `[SimConnect]` section.
pub fn parse(contents: &str) -> HashMap<u32, CfgEntry> {
    let mut sections = HashMap::new();
    let mut current: Option<u32> = None;
    let mut address = String::new();
    let mut port = 0u16;

    let flush =
        |sections: &mut HashMap<u32, CfgEntry>, current: Option<u32>, address: &str, port: u16| {
            if let Some(idx) = current {
                if !address.is_empty() && port != 0 {
                    sections.insert(
                        idx,
                        CfgEntry {
                            address: address.to_string(),
                            port,
                        },
                    );
                }
            }
        };

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            flush(&mut sections, current, &address, port);
            let name = &line[1..line.len() - 1];
            current = if name.eq_ignore_ascii_case("SimConnect") {
                Some(0)
            } else if let Some(suffix) = name.to_ascii_lowercase().strip_prefix("simconnect.") {
                suffix.parse().ok()
            } else {
                None
            };
            address.clear();
            port = 0;
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim().to_ascii_lowercase().as_str() {
                "address" => address = value.trim().to_string(),
                "port" => port = value.trim().parse().unwrap_or(0),
                _ => {}
            }
        }
    }
    flush(&mut sections, current, &address, port);
    sections
}

/// Searches `default_search_paths()` for a usable `SimConnect.cfg` and
/// returns entry `index` (0 = default) if found.
pub fn discover(index: u32) -> Option<CfgEntry> {
    for path in default_search_paths() {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Some(entry) = parse(&contents).remove(&index) {
                return Some(entry);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_section() {
        let cfg = "[SimConnect]\nAddress=192.168.1.50\nPort=500\n";
        let sections = parse(cfg);
        assert_eq!(
            sections[&0],
            CfgEntry {
                address: "192.168.1.50".into(),
                port: 500
            }
        );
    }

    #[test]
    fn parses_indexed_sections() {
        let cfg = "[SimConnect.1]\nAddress=10.0.0.2\nPort=1234\n";
        let sections = parse(cfg);
        assert_eq!(sections[&1].port, 1234);
    }

    #[test]
    fn ignores_incomplete_sections() {
        let cfg = "[SimConnect]\nAddress=only-address\n";
        assert!(parse(cfg).is_empty());
    }

    #[test]
    fn default_search_paths_returns_paths() {
        let paths = default_search_paths();
        assert!(!paths.is_empty());
    }
}
