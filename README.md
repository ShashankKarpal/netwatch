# Netwatch

A trust-aware network monitor for macOS. Built on top of [Sniffnet](https://github.com/GyulyVGC/sniffnet) by [Giuliano Bellini](https://github.com/GyulyVGC).

## What this does

Netwatch answers one question at a glance: **"Is my Mac doing anything on the network that I do not expect right now?"**

It classifies every active connection into three buckets:

- **Expected** (green): App-to-host pairings you have seen before and marked as normal
- **New** (amber): Any app-to-host pairing seen for the first time
- **Flagged** (red): Connections matching rules you set (e.g., "LM Studio should never connect to anything except localhost")

This is not a firewall. It does not block anything. It gives you visibility and judgment in seconds, not minutes.

## Why this exists

Modern Macs run dozens of background processes making network connections: browsers, chat apps, cloud sync, update checkers, telemetry, local AI tools. There is no simple way to know whether all of this is expected or whether something is quietly phoning home.

Existing tools either show raw packet data (unusable without deep networking knowledge) or cost money and require complex configuration. Netwatch fills the gap: free, open source, and opinionated toward trust classification.

## Development philosophy

This project follows a behavior-first development framework. Every version must prove that a specific human behavior has changed. Not "I added a feature." But "the user is now doing X that they were not doing before."

The full user loop:

**Entry > Awareness > Judgment > Action > Return > Habit**

Each version strengthens one part of this loop. If a feature does not strengthen the loop, it does not belong in that version.

## Roadmap

| Version | Loop stage | Behavior to prove | Status |
|---------|------------|-------------------|--------|
| v0.1 | Entry | I open this app at least once per work session without being reminded, because it answers a question I already have | In progress |
| v0.2 | Return | After a triggering event (new app, VPN switch, new LLM model), I reopen the app because last time it told me something useful | Not started |
| v0.3 | Action | I take a concrete action (investigate, flag, mark safe) because of something the app surfaced | Not started |
| v1.0 | Habit | This is now part of how I operate my Mac. I check it after installing anything new | Not started |

## Build instructions

Requires Rust and Cargo. macOS has all native dependencies (libpcap ships with the OS).
cargo build --release
sudo cargo run --release

Root privileges are required for packet capture on macOS. The app will prompt for your system password on launch.

## Credits

Netwatch is a fork of [Sniffnet](https://github.com/GyulyVGC/sniffnet), created and maintained by [Giuliano Bellini](https://github.com/GyulyVGC). Full credit to Giuliano for building Sniffnet as a free, open-source network monitor over 4 years and 2,700+ commits. The packet capture engine, GUI framework, protocol analysis, and geolocation features are entirely his work.

Netwatch adds a trust classification layer on top of Sniffnet's foundation. This fork exists to experiment freely, not to fragment the project. Any changes worth contributing back will go upstream as PRs.

## License

Dual-licensed under MIT and Apache 2.0, same as upstream.
