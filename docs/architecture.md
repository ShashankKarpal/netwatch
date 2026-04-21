# Architecture

## Upstream (Sniffnet)

- Written in Rust
- GUI built with iced (cross-platform Rust GUI library)
- Packet capture via libpcap (native on macOS, no additional install)
- Real-time traffic display: per-host, per-service, per-app bandwidth
- Runs with root privileges (sudo) for packet capture access
- No persistent storage of connection history in upstream

## Key source files (mapped during audit)

### Data model
- src/networking/types/address_port_pair.rs — Connection identity (source IP, dest IP, ports, protocol)
- src/networking/types/info_address_port_pair.rs — Connection metadata (program, service, direction, bytes, timestamps, blacklist status)
- src/networking/types/program.rs — Program enum: NamePath(name, path), Unknown, or NotApplicable
- src/networking/types/host.rs — Host info (domain from reverse DNS, ASN, country)
- src/networking/types/data_info_host.rs — Host flags (is_loopback, is_local, is_bogon)

### GUI rendering
- src/gui/pages/inspect_page.rs — The connection list page
  - Line 91: report() loops through connections
  - Line 107: calls row_report_entry(key, val, data_repr) for each connection
  - Line 247: row_report_entry() builds each row using ReportCol columns
  - THIS IS THE INJECTION POINT for trust badges

### Other relevant directories
- src/gui/components/ — Reusable UI widgets (buttons, header, footer, tabs, modals)
- src/gui/styles/ — Visual styling (colors, text types, button types, container types)
- src/gui/types/ — GUI state types (messages, settings)
- src/report/ — Report generation, search, filtering, sorting

## Netwatch additions (v0.1 plan)

### New module: src/trust/

- trust_db.rs — Load, save, and query the trust database
- trust_rules.rs — Load and evaluate flagging rules
- trust_level.rs — Enum: Expected, New, Flagged
- mod.rs — Module exports

### Trust database

- Location: ~/.config/netwatch/trust.json
- Schema: array of { program: string, dest_ip: string, level: "expected" | "flagged" }
- Any (program, dest_ip) pair not in the database is classified as New
- Bootstrap: first run shows everything as New. User marks connections over first few sessions.

### Flagging rules

- Location: ~/.config/netwatch/rules.json
- Schema: array of { program: string, allowed_hosts: [string] }
- Example: { "program": "LM Studio", "allowed_hosts": ["127.0.0.1", "localhost"] }
- Any connection from a ruled program to a host not in allowed_hosts triggers Flagged (red)

### Trust View UI

- Adds a colored circle (green/amber/red) as the first column in each row of row_report_entry()
- Green: Expected. Amber: New. Red: Flagged.
- Clicking a New connection offers "Mark as Expected" action

## Known constraints

- iced GUI framework has its own layout and widget patterns. UI changes must follow iced conventions.
- Sniffnet uses ReportCol enum to define columns. Adding a trust column means extending this pattern.
- The trust database must be readable/writable by the user without root, even though the app runs with sudo.
- Host domain (reverse DNS) is resolved separately from the connection row data. Trust classification will initially use dest_ip, with domain matching added later.
