// TrustDb is implemented but not yet wired into the packet pipeline.
// See the fork gates in ROADMAP.md. Remove this attribute once classify()
// is called from sniffer.rs and these items go live.
#![expect(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// A single trusted (program, destination) pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrustEntry {
    pub program: String,
    pub dest: String,
}

/// The trust database: a set of known (program, dest) pairs.
#[derive(Debug, Clone, Default)]
pub struct TrustDb {
    pub entries: HashSet<TrustEntry>,
    path: PathBuf,
}

impl TrustDb {
    /// Load trust database from disk, or create empty if not found.
    pub fn load() -> Self {
        let path = Self::db_path();
        let entries = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashSet::new()
        };
        TrustDb { entries, path }
    }

    /// Check if a (program, dest) pair is in the trust database.
    pub fn is_expected(&self, program: &str, dest: &str) -> bool {
        self.entries.contains(&TrustEntry {
            program: program.to_string(),
            dest: dest.to_string(),
        })
    }

    /// Mark a (program, dest) pair as expected and save to disk.
    pub fn mark_expected(&mut self, program: String, dest: String) {
        self.entries.insert(TrustEntry { program, dest });
        self.save();
    }

    /// Save the trust database to disk.
    pub fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let _ = fs::write(&self.path, json);
        }
    }

    /// Path to the trust database file.
    fn db_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("netwatch")
            .join("trust.json")
    }
}
