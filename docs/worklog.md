# Worklog

## 2026-04-21

- Forked sniffnet to ShashankKarpal/netwatch
- Installed Rust 1.95.0 (already present)
- Cloned to ~/Documents/GitHub/netwatch via GitHub Desktop
- First build: cargo build --release (success, 3m 43s on M4 Pro)
- First run: sudo cargo run --release (app launched, GUI displayed correctly)
- Created v0.1-trust-view branch
- Pushed README.md with upstream credit and behavior-first roadmap
- Created CLAUDE.md and docs/ markdown OS (9 files)
- Codebase audit completed:
  - Data model: AddressPortPair (connection identity), InfoAddressPortPair (metadata incl Program), Host (domain, ASN)
  - GUI pipeline: inspect_page.rs > report() > row_report_entry() at line 247
  - Injection point confirmed: row_report_entry() receives key (AddressPortPair) and val (InfoAddressPortPair)
- Created src/trust/ module with 4 files:
  - trust_level.rs: TrustLevel enum (Expected, New, Flagged)
  - trust_db.rs: JSON-backed trust database at ~/.config/netwatch/trust.json
  - trust_rules.rs: JSON-backed flagging rules at ~/.config/netwatch/rules.json
  - mod.rs: classify() function combining both
- Added dirs v6.0.0 crate dependency
- Wired TrustDb and TrustRules into Sniffer struct (sniffer.rs)
- Modified inspect_page.rs: added trust imports, passed trust data to row_report_entry(), added emoji badge column
- First compile error: iced Text::color() API incompatibility. Fixed by switching to emoji badges instead of colored text.
- MILESTONE: Live test successful. All connections show yellow (New) badges in Inspect tab. Trust View is working.
