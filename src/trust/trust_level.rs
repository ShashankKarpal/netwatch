/// Trust classification for a network connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    /// Known and approved connection
    Expected,
    /// Never seen before
    New,
    /// Matches a flagging rule (e.g., app connecting to a disallowed host)
    Flagged,
}
