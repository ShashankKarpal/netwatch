# Current Phase

## Version: v0.1 — Trust View

## Loop stage: Entry

## Behavior to prove

Shanky opens this app at least once per work session without being reminded, because it answers a question he already has: "Is my Mac doing anything unexpected on the network right now?"

## What v0.1 includes

- A Trust View overlay on top of Sniffnet's existing traffic data
- Three classification buckets: Expected (green), New (amber), Flagged (red)
- A local JSON file storing known app-to-host pairings (the trust database)
- Manual rules for flagging (e.g., "LM Studio connects only to localhost")
- Ability to mark a New connection as Expected (right-click or button)

## Completion criterion

Shanky opens the app 3+ times in a week without being prompted, because the trust view answers a real question.

## Explicitly out of scope for v0.1

- Firewall or blocking rules (no dropping packets)
- macOS notifications or alerts
- Historical logging or SQLite persistence
- Per-VPN-interface traffic breakdown
- Bandwidth charts or usage history
- MCP server isolation view
- Automation of any kind
- Any UI changes beyond the Trust View overlay

## Why these are out of scope

Each of these belongs to a later loop stage (Return, Action, or Habit). Building them now would add complexity without proving the entry behavior. If the user does not open the app voluntarily, none of these features matter.
