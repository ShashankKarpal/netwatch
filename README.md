<h1 align="center">Netwatch</h1>

<p align="center"><b>A trust-aware fork of Sniffnet that classifies every connection as Expected, New, or Flagged.</b></p>

<p align="center">
  <img alt="Fork" src="https://img.shields.io/badge/fork%20of-GyulyVGC%2Fsniffnet-1C1B1D?style=flat-square">
  <img alt="Platform" src="https://img.shields.io/badge/platform-macOS-1C1B1D?style=flat-square">
  <img alt="Status" src="https://img.shields.io/badge/status-v0.1%20trust%20view-1C1B1D?style=flat-square">
  <img alt="Stack" src="https://img.shields.io/badge/built%20with-Rust-1C1B1D?style=flat-square">
  <a href="LICENSE-MIT"><img alt="License" src="https://img.shields.io/badge/license-MIT%20%C2%B7%20Apache--2.0-1C1B1D?style=flat-square"></a>
</p>

## Attribution

Forked from [GyulyVGC/sniffnet](https://github.com/GyulyVGC/sniffnet). Full credit to [Giuliano Bellini](https://github.com/GyulyVGC) for building Sniffnet and maintaining it as a free, open-source network monitor. Everything below describes only what this fork adds; every other feature, and all of the packet-capture engine, is upstream work. Changes worth contributing back will be submitted as pull requests rather than kept here.

## What this fork adds

- Classifies every active connection into Expected, New, or Flagged.
- Shows the classification as a colored badge inline in the Inspect tab.
- Remembers app-to-host pairings you have marked normal, in a local trust database.
- Answers one question at a glance: is this Mac doing anything on the network that you do not expect.

## Features

### Trust layer (this fork)

- **Trust classification.** Every connection resolves to Expected (green), New (amber), or Flagged (red).
- **Trust database.** App-to-host pairings persist locally between sessions.
- **Trust rules.** User-defined rules flag connections that should never happen, for example a local model server reaching anything other than localhost.
- **Inline badges.** Classification renders per row in the Inspect tab, not on a separate screen.
- **Empty-state behavior.** With no trust database, everything reads New, which is the correct default.

### Not in this fork

- No firewall and no blocking. This is visibility, not enforcement.
- No historical logging, per-VPN breakdown, bandwidth charts, notifications, or automation. All deliberately out of scope for v0.1.

## Stack

- Language: Rust
- Capture: libpcap, ships with macOS
- Fork-specific module: `src/trust/` (TrustLevel, TrustDb, TrustRules, classify)

## Install

Requires: Rust and Cargo. macOS has all native dependencies.

```bash
git clone https://github.com/ShashankKarpal/netwatch.git
cd netwatch
cargo build --release
sudo cargo run --release
```

Root privileges are required for packet capture; macOS will prompt on launch.

## Project structure

```
src/trust/          this fork: TrustLevel, TrustDb, TrustRules, classify()
src/gui/            upstream GUI, with trust badges injected in inspect_page.rs
src/networking/     upstream capture and parsing
resources/          upstream assets
```

## Roadmap

Every version must prove one human behavior changed, not that a feature shipped.

| Version | Loop stage | Behavior to prove | Status |
|---|---|---|---|
| v0.1 | Entry | The app is opened once per work session, unprompted, because it answers a question already being asked | In progress |
| v0.2 | Return | The app is reopened after a triggering event, because last time it was useful | Planned |
| v0.3 | Action | A concrete action is taken because of something the app surfaced | Planned |
| v1.0 | Habit | The app is checked after installing anything new | Planned |

## License

MIT and Apache-2.0, inherited from upstream. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## Author

Fork maintained by Shashank Karpal. Upstream by Giuliano Bellini.

> Trust layer designed and built with Claude (Anthropic).
