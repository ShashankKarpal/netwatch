# Decision Log

## 2026-04-21 — Fork Sniffnet instead of building from scratch

**Context:** Needed a network monitor with trust classification. Options were: build from scratch, fork Sniffnet, or fork RustNet Monitor.
**Decision:** Fork Sniffnet.
**Reason:** 33k stars, actively maintained, clean Rust codebase, MIT/Apache dual license, native macOS support with zero additional dependencies. Building from scratch would take months for the same baseline. Sniffnet already solves Entry and Awareness; we only need to add Judgment.

## 2026-04-21 — Behavior-first development framework

**Context:** Needed a development approach that prevents feature creep and ensures each version proves real value.
**Decision:** Adopt Karl's behavior-first framework. One behavior per version, review gate before the next.
**Reason:** Proven effective on the content digest app. Prevents the common failure mode of building features nobody uses.

## 2026-04-21 — Name the fork Netwatch

**Context:** Needed a distinct name for the fork.
**Decision:** Netwatch.
**Reason:** Short, descriptive, not already taken on GitHub under this account. Communicates "watching the network" without being generic.

## 2026-04-21 — Trust View as v0.1 scope

**Context:** Many possible features (firewall, VPN audit, bandwidth charts, alerts). Needed to pick one.
**Decision:** Trust View (Expected / New / Flagged classification) is the entire v0.1 scope.
**Reason:** This is the smallest change that proves the entry behavior. If the user cannot form a judgment at a glance, nothing else matters. Firewall rules, alerts, and history all depend on the user opening the app first.
