use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// A rule that restricts which hosts a program may connect to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagRule {
    pub program: String,
    pub allowed_hosts: Vec<String>,
}

/// Collection of flagging rules.
#[derive(Debug, Clone, Default)]
pub struct TrustRules {
    rules: HashMap<String, Vec<String>>,
}

impl TrustRules {
    /// Load rules from disk, or return empty if not found.
    pub fn load() -> Self {
        let path = Self::rules_path();
        let rules_vec: Vec<FlagRule> = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        let mut rules = HashMap::new();
        for rule in rules_vec {
            rules.insert(rule.program.to_lowercase(), rule.allowed_hosts);
        }
        TrustRules { rules }
    }

    /// Check if a program is flagged for connecting to a given destination.
    /// Returns true if the program has a rule AND the dest is NOT in the allowed list.
    pub fn is_flagged(&self, program: &str, dest: &str) -> bool {
        if let Some(allowed) = self.rules.get(&program.to_lowercase()) {
            !allowed.iter().any(|h| h == dest)
        } else {
            false
        }
    }

    /// Path to the rules file.
    fn rules_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("netwatch")
            .join("rules.json")
    }
}
