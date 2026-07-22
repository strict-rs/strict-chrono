## Constructors, timestamps, parsing

- Low-level: `from_naive_utc_and_offset` (const, canonical). `from_utc`/`from_local` are deprecated.
- `DateTime<Utc>` timestamp constructors are all `const`: `from_timestamp_secs`, `from_timestamp(secs, nsecs)`, `from_timestamp_millis`/`from_timestamp_micros` (→ `Option`), and `from_timestamp_nanos` (infallible — every `i64` nanosecond value is in range). Associated consts: `MIN_UTC`, `MAX_UTC`, `UNIX_EPOCH`; `UNIX_EPOCH_DAY` (`pub(crate)`, `719_163`) backs `timestamp()`/`from_timestamp()`.
- `timestamp_nanos_opt` carries a negative-timestamp underflow workaround (`((ts+1)*1e9)+(ns-1e9)`, chrono#1289); `timestamp_nanos()` is the deprecated panicking form. Subsecond accessors can exceed their nominal maxima during a leap second (`secs % 60 == 59`).
- Parsing lives on `DateTime<FixedOffset>` and **requires an offset in the input**: `parse_from_rfc2822`, `parse_from_rfc3339`, `parse_from_str`, `parse_and_remainder`. `FromStr` for `DateTime<Utc>`/`DateTime<Local>` parses a relaxed RFC 3339 (space or `T` separator) as `FixedOffset`, then converts.
