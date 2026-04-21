pub mod trust_db;
pub mod trust_level;
pub mod trust_rules;

use crate::trust::trust_db::TrustDb;
use crate::trust::trust_level::TrustLevel;
use crate::trust::trust_rules::TrustRules;

/// Classify a connection based on the trust database and flagging rules.
pub fn classify(
    program: &str,
    dest: &str,
    trust_db: &TrustDb,
    rules: &TrustRules,
) -> TrustLevel {
    // Flagging rules take priority
    if rules.is_flagged(program, dest) {
        return TrustLevel::Flagged;
    }
    // Check trust database
    if trust_db.is_expected(program, dest) {
        return TrustLevel::Expected;
    }
    // Default: never seen before
    TrustLevel::New
}
