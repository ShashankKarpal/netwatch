# Roadmap

## v0.1 — Entry (current)

Prove the user opens the app voluntarily because it answers a real question.
Core deliverable: Trust View (Expected / New / Flagged classification).

## v0.2 — Return

Prove the user reopens the app after a triggering event (new app installed, VPN toggled, new LLM model loaded).
Core deliverable: Lightweight macOS notification when a New connection appears while the app runs in the background.

## v0.3 — Action

Prove the user takes concrete action (investigates, flags, marks safe) because of what the app surfaced.
Core deliverable: Right-click context menu on connections (Mark Expected, Flag This App, Copy Details). Trust database grows from real usage.

## v1.0 — Habit

Prove this is now part of how the user operates their Mac.
Core deliverable: VPN interface awareness, persistent SQLite history, and possibly lightweight block rules via macOS packet filter (pf). Only if v0.1 through v0.3 are proven.

## Rules

- No version starts until the previous version's behavior is proved
- Each version has a review gate before the next begins
- Features that do not strengthen the current loop stage are deferred, not deleted
