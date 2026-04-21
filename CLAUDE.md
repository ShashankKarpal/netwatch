# CLAUDE.md — Netwatch

## What this is

Netwatch is a forked and customized version of Sniffnet (github.com/GyulyVGC/sniffnet), a Rust-based open-source network traffic monitor. This fork adds a Trust View layer that classifies every network connection as Expected, New, or Flagged, so the user can answer one question at a glance: "Is my Mac doing anything on the network that I do not expect right now?"

## Who this is for

One user: Shanky. Mac-based, privacy-first, runs local LLMs, multiple VPNs, MCP servers, and Chrome extensions. Non-developer. All code is written by Claude and executed via Terminal.

## Development philosophy

This project follows a behavior-first development framework. Every version must prove that a specific human behavior has changed. Not "I added a feature." But "users are now doing X that they were not doing before."

The full user loop: Entry > Awareness > Judgment > Action > Return > Habit.

Each version strengthens one part of this loop. If a feature does not strengthen the loop, it does not belong in that version.

## Before doing any substantial work

Read at minimum:
- This file (CLAUDE.md)
- docs/product-intent.md
- docs/current-phase.md
- docs/session-handoff.md
- docs/todo.md

## Operating rules

1. Do not overbuild. Do not add features that do not prove the current version's target behavior.
2. Audit before features. Before adding anything new, confirm the current loop works end to end.
3. Update docs/session-handoff.md at the end of every meaningful session.
4. Update docs/decision-log.md when a real decision is made.
5. Do not drift from the core user loop. If a proposed change strengthens none of the loop stages, say so explicitly.
6. All code changes must be explained step by step. Shanky runs commands in Terminal and reports output. Never assume coding familiarity.
7. Use Terminal heredoc or python3 -c for file edits. Never use TextEdit (causes indentation errors).

## Tech stack

- Language: Rust
- GUI: iced (cross-platform Rust GUI library)
- Packet capture: libpcap (native on macOS)
- Build: cargo build --release
- Run: sudo cargo run --release (root required for packet capture)
- Local path: ~/Documents/GitHub/netwatch
- Upstream: github.com/GyulyVGC/sniffnet
