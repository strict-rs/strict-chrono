## serde (`serde.rs`, feature `serde`)

The default `Serialize` writes an RFC 3339 string (`SecondsFormat::AutoSi`, `use_z = true`); `Deserialize` is provided for `FixedOffset`/`Utc`/`Local` through a shared `DateTimeVisitor`. The integer-timestamp adapters are `pub mod`s re-exported at `crate::serde`, for use with `#[serde(with = "...")]`: `ts_seconds`, `ts_milliseconds`, `ts_microseconds`, `ts_nanoseconds`, and their `_option` variants (over `Option<DateTime<Utc>>`). All operate on `DateTime<Utc>`. `ts_nanoseconds` serialize **errors** on out-of-range values rather than panicking.
