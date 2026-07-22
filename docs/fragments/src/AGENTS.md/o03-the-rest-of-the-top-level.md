## The rest of the top level

- `time_delta.rs` — `TimeDelta` (re-exported as the type alias `Duration`), a signed span stored as `secs: i64` + `nanos: i32`. Its range is clamped to ±`i64::MAX` **milliseconds**, and the minimum is `-i64::MAX` rather than `i64::MIN` specifically so `abs()`/negation can never overflow. Note the two constructor families: the `try_*` ones return `Option`, while the same-named non-`try` ones (`weeks`/`days`/`hours`/…) panic on overflow.
- `traits.rs` — the `Datelike` and `Timelike` field-accessor traits. The doc-comment warnings matter: chaining two `with_*` calls can fail on a non-existent intermediate value, so callers should reconstruct via `NaiveDate` instead.
- `month.rs` / `weekday.rs` / `weekday_set.rs` — `Month`/`Months`, `Weekday`, and `WeekdaySet`. `Month` is stored zero-indexed but its public API is 1-indexed. `Weekday` intentionally implements neither `PartialOrd` nor `Ord` (the "first" day depends on context — use the `*_from_monday`/`*_from_sunday` methods). `WeekdaySet` is a `u8` bitmask with the invariant that its 8th bit is always 0.
- `round.rs` — the `SubsecRound` and `DurationRound` extension traits plus `RoundingError`.
- `date.rs` — the deprecated `Date<Tz>` type, kept only to support the deprecated `TimeZone::ymd`-style constructors; the whole file is under `#![allow(deprecated)]`.
