# Session Handoff

## Last updated: 2026-04-21

## Current branch: v0.1-trust-view

## What was done this session

- Forked GyulyVGC/sniffnet to ShashankKarpal/netwatch
- Cloned locally to ~/Documents/GitHub/netwatch
- Verified build succeeds (cargo build --release, 3m 43s)
- Verified app launches with sudo cargo run --release
- Created v0.1-trust-view branch
- Added FORK_README.md with upstream credit and behavior-first roadmap
- Created CLAUDE.md and full docs/ folder (markdown OS)
- Completed codebase audit: mapped data model, GUI rendering pipeline, and exact injection point
- Key finding: row_report_entry() at line 247 of inspect_page.rs is where trust badges get added
- Key finding: InfoAddressPortPair.program gives us the app name, AddressPortPair.dest gives us the destination IP

## What is next

1. Create the src/trust/ module (trust_db.rs, trust_rules.rs, trust_level.rs)
2. Define the TrustLevel enum (Expected, New, Flagged)
3. Build the trust database loader (read/write ~/.config/netwatch/trust.json)
4. Build the rules loader (read ~/.config/netwatch/rules.json)
5. Write the classify() function that takes (program, dest_ip) and returns a TrustLevel
6. Inject trust badge into row_report_entry() in inspect_page.rs
7. Add "Mark as Expected" action on New connections
8. Create default rules.json with LM Studio localhost-only rule
9. Test with real traffic

## Blockers

None.

## Warnings

- Shanky is non-technical. All code must be written by Claude and delivered as Terminal commands.
- Do not skip testing after each code change. Build and run after every modification.
- The iced GUI framework requires following its widget patterns. Do not use raw HTML or CSS thinking.
- Trust database writes must work without root even though the app runs with sudo.
