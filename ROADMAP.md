# Roadmap

This fork's roadmap. Upstream Sniffnet's own roadmap lives at the
[Sniffnet project](https://github.com/GyulyVGC/sniffnet) and is not duplicated here.

Every version must prove one human behavior changed, not that a feature shipped.
No version starts until the previous version's behavior is proved, and each
version has a review gate before the next begins. Features that do not
strengthen the current loop stage are deferred, not deleted.

## v0.1, Entry (current)

Prove the app is opened voluntarily because it answers a real question.
Core deliverable: Trust View (Expected / New / Flagged classification).

## v0.2, Return

Prove the app is reopened after a triggering event (new app installed, VPN
toggled, new LLM model loaded).
Core deliverable: lightweight macOS notification when a New connection appears
while the app runs in the background.

## v0.3, Action

Prove a concrete action is taken (investigate, flag, mark safe) because of
what the app surfaced.
Core deliverable: "Mark as Expected" write path with per-row trust controls in
the Inspect tab. Trust database grows from real usage.

## v1.0, Habit

Prove this is now part of how the Mac is operated.
Core deliverable: VPN interface awareness, persistent SQLite history, and
possibly lightweight block rules via the macOS packet filter (pf). Only if
v0.1 through v0.3 are proven.
