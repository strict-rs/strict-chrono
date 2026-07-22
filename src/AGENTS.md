<!-- Do not edit; generated file. -->

# src/ — crate root and shared primitives

`lib.rs` is the crate root: it owns feature gating (`no_std` when neither `std` nor `test` is active), the `prelude`, the public re-export surface, and two small but pervasive helpers — the `try_opt!` macro and the `pub(crate) const fn expect()`, which stand in for `?` and `.expect()` in `const` context (that is why so much of the crate threads `Option` through `try_opt!` by hand).

## Where the big subsystems live — see the child guides

The four subsystems each have their own `AGENTS.md`; consult those rather than duplicating their internals here:

- `datetime/` — `DateTime<Tz>`, the timezone-aware instant. See `datetime/AGENTS.md`.
- `format/` — the shared `strftime` parse/format engine (`Item`, `StrftimeItems`, `Parsed`). See `format/AGENTS.md`.
- `naive/` — the timezone-naive types and their bit-packed representation. See `naive/AGENTS.md`.
- `offset/` — the `TimeZone`/`Offset` traits, `MappedLocalTime`, and `Utc`/`FixedOffset`/`Local`. See `offset/AGENTS.md`.

## The rest of the top level

- `time_delta.rs` — `TimeDelta` (re-exported as the type alias `Duration`), a signed span stored as `secs: i64` + `nanos: i32`. Its range is clamped to ±`i64::MAX` **milliseconds**, and the minimum is `-i64::MAX` rather than `i64::MIN` specifically so `abs()`/negation can never overflow. Note the two constructor families: the `try_*` ones return `Option`, while the same-named non-`try` ones (`weeks`/`days`/`hours`/…) panic on overflow.
- `traits.rs` — the `Datelike` and `Timelike` field-accessor traits. The doc-comment warnings matter: chaining two `with_*` calls can fail on a non-existent intermediate value, so callers should reconstruct via `NaiveDate` instead.
- `month.rs` / `weekday.rs` / `weekday_set.rs` — `Month`/`Months`, `Weekday`, and `WeekdaySet`. `Month` is stored zero-indexed but its public API is 1-indexed. `Weekday` intentionally implements neither `PartialOrd` nor `Ord` (the "first" day depends on context — use the `*_from_monday`/`*_from_sunday` methods). `WeekdaySet` is a `u8` bitmask with the invariant that its 8th bit is always 0.
- `round.rs` — the `SubsecRound` and `DurationRound` extension traits plus `RoundingError`.
- `date.rs` — the deprecated `Date<Tz>` type, kept only to support the deprecated `TimeZone::ymd`-style constructors; the whole file is under `#![allow(deprecated)]`.

## Cross-cutting patterns worth knowing

- **Const-context plumbing.** `try_opt!` / `expect()` appear everywhere because most constructors are `const fn` and cannot use `?` / `.expect()`.
- **Leap seconds** are represented uniformly as a nanosecond field ≥ 1e9 (only when `secs % 60 == 59`); the details live in `naive/` and `format/`.
- **The out-of-range buffer.** `NaiveDate` reserves one year of headroom (`BEFORE_MIN`/`AFTER_MAX`) so a local time just outside the representable range can exist transiently during offset math without panicking; `DateTime::overflowing_naive_local` and `NaiveDateTime::overflowing_*_offset` rely on it. Never expose such a value.
- **Feature gating.** `alloc` unlocks string formatting, `std` adds OS/`SystemTime` interop, `clock` enables `Local`, and the `rkyv-16`/`rkyv-32`/`rkyv-64` features are mutually exclusive. Optional integrations (`serde`, `rkyv`, `arbitrary`, `defmt`) are gated at the crate root and mirrored per type.
