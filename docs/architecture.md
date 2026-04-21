# Architecture

## Upstream (Sniffnet)

- Written in Rust
- GUI built with iced (cross-platform Rust GUI library)
- Packet capture via libpcap (native on macOS, no additional install)
- Real-time traffic display: per-host, per-service, per-app bandwidth
- Runs with root privileges (sudo) for packet capture access
- No persistent storage of connection history in upstream

## Netwatch additions (v0.1 plan)

### Trust database

- A local JSON file at ~/.config/netwatch/trust.json
- Stores known app-to-host pairings with their classification (expected, flagged)
- Any pairing not in the database is classified as New
- Bootstrap: first run will show everything as New. User marks connections as Expected over the first few sessions.

### Trust View overlay

- Adds a color-coded classification column or badge to Sniffnet's existing connection list
- Green: Expected (known and approved pairing)
- Amber: New (never seen before)
- Red: Flagged (matches a user-defined rule)

### Flagging rules

- Simple JSON rules in ~/.config/netwatch/rules.json
- Format: { "app": "LM Studio", "allowed_hosts": ["localhost", "127.0.0.1"] }
- Any connection from a flagged app to a host not in its allowed list triggers red status

## Known constraints

- Sniffnet uses iced for GUI, which has its own layout and widget system. Adding UI elements requires working within iced's framework.
- The codebase is well-structured but moderately large (~2,700 commits). Understanding where to inject the trust layer requires a codebase audit.
- macOS requires root for packet capture. The trust database and rules files should be readable/writable by the user without root.

## Key directories in the codebase

To be mapped during the v0.1 codebase audit (first implementation task).
