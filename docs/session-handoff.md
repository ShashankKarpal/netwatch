# Session Handoff

## Last updated: 2026-04-21

## Current branch: v0.1-trust-view

## What was done this session

- Forked GyulyVGC/sniffnet to ShashankKarpal/netwatch
- Cloned locally to ~/Documents/GitHub/netwatch
- Verified build succeeds (cargo build --release)
- Verified app launches with sudo cargo run --release
- Created v0.1-trust-view branch
- Added README.md with upstream credit and behavior-first roadmap
- Created CLAUDE.md and full docs/ folder (markdown OS)
- Completed codebase audit: mapped data model, GUI rendering pipeline, and injection point
- Created src/trust/ module: TrustLevel enum, TrustDb, TrustRules, classify()
- Added dirs crate dependency for config directory paths
- Wired TrustDb and TrustRules into Sniffer state (sniffer.rs)
- Injected trust badges into row_report_entry() in inspect_page.rs
- First live test: all connections show yellow (New) badges as expected with empty trust database
- Trust View is visually confirmed working in the Inspect tab

## What is next

1. Create the default rules.json with LM Studio localhost-only rule
2. Add "Mark as Expected" action (clicking a New connection saves it to trust.json)
3. Test marking a few connections as Expected and confirm green badges appear
4. Add trust badge to the report header row (column label)
5. Test with LM Studio running to verify flagging logic (red badges)
6. Run a full work session and evaluate: did I open the app voluntarily 3+ times?

## Blockers

None.

## Warnings

- Emoji badges work but are a temporary solution. Later versions may use iced's native styling for colored dots.
- The trust database at ~/.config/netwatch/trust.json does not exist yet. It will be created on first "Mark as Expected" action.
- Rules file at ~/.config/netwatch/rules.json does not exist yet. Needs to be seeded with defaults.
- Remember to merge v0.1-trust-view into main periodically so the GitHub repo shows current state.
