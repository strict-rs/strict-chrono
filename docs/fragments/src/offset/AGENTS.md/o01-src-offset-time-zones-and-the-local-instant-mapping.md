# src/offset — time zones and the local↔instant mapping

Owns the `TimeZone` and `Offset` traits, the `MappedLocalTime` result type, and the three concrete zones: `Utc`, `FixedOffset`, and `Local`. `mod.rs` holds the traits and the primary `DateTime` constructors; `fixed.rs`/`utc.rs` are the two trivial zones; `local/` is the large platform-specific machinery.
