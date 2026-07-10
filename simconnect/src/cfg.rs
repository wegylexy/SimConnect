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

/// Standard search locations, in order, for a `SimConnect.cfg` next to the
/// running executable or in the user's `%APPDATA%` profile.
pub fn default_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join("SimConnect.cfg"));
        }
    }
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(
            Path::new(&appdata)
                .join("Microsoft Flight Simulator")
                .join("SimConnect.cfg"),
        );
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

    let flush = |sections: &mut HashMap<u32, CfgEntry>,
                 current: Option<u32>,
                 address: &str,
                 port: u16| {
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
            } else if let Some(suffix) = name
                .to_ascii_lowercase()
                .strip_prefix("simconnect.")
            {
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
}
