# src/datetime — `DateTime<Tz>`

Owns the timezone-aware combined date-and-time type `DateTime<Tz: TimeZone>`, its serde integration (`serde.rs`, gated on the `serde` feature), and its tests (`tests.rs`).

## Core representation, and what follows from it

`DateTime<Tz>` holds exactly two fields: `datetime: NaiveDateTime` — **always in UTC** — and `offset: Tz::Offset`. No `Tz` value is stored, only its `Offset`, so the timezone object is reconstructed on demand via `TimeZone::from_offset`, and `Tz` never gates `Copy`/`Send`. The offset never changes the stored instant; it only derives the local wall-clock view, computed on demand as `datetime + offset.fix()`.

Two consequences drive most of the surprises here:

- **Equality, ordering, and hashing use the UTC instant only** — the offset is deliberately ignored. `PartialEq`/`PartialOrd` are cross-`Tz` (`DateTime<Tz>` vs `DateTime<Tz2>`) and compare `self.datetime` alone, so two values with different offsets but the same instant are `==` and hash-equal. Tests that also need to assert the offset compare it separately (a `norm()` helper).
- **The local view can fall just outside `NaiveDateTime`'s representable range** at the extremes, so there are two accessors. `naive_local()` **panics** when the local value is out of range; `overflowing_naive_local()` (`pub(crate)`) uses buffer space beyond the normal range and never panics. Nearly everything reads through the overflowing form — the `Datelike`/`Timelike` getters, `Debug`/`Display`, `format*`, `to_rfc2822`, and `to_rfc3339` — while `to_rfc3339_opts()` and the serde serializer use the panicking `naive_local()`. Its doc contract is explicit: the overflowing value must **never be exposed outside chrono**.

## Panics vs `Option`

Fallible operations split cleanly: the `checked_*` methods (`checked_add/sub_signed`, `checked_add/sub_months`, `checked_add/sub_days`) and `years_since` return `Option`, while `signed_duration_since` returns a `TimeDelta` and never overflows; `naive_local()` and the arithmetic operators (`Add`/`Sub`/`AddAssign`/`SubAssign`) panic. Every panicking operator is `#[track_caller]` and delegates to its `checked_*` sibling with `.expect(...)`. `Add`/`Sub` of a `FixedOffset` shift the stored UTC `datetime` and leave the `offset` field unchanged.

`with_*` setters (from `Datelike`/`Timelike`) route through the private `map_local` helper: it applies the change to `overflowing_naive_local()`, re-resolves through `timezone().from_local_datetime(..).single()`, then filters to `MIN_UTC..=MAX_UTC`. That is why a setter returns `None` for a nonexistent or ambiguous local time (a DST transition) or an out-of-range result.

Subtlety to preserve: `checked_add_months`/`checked_add_days` (and the `sub` forms) intentionally exploit the `Months(0)`/`Days(0)` fast paths in `NaiveDate`, which skip validation, so a zero-length add can return `Some` even when the local datetime is out of range. `checked_add_days` additionally short-circuits `Days::new(0) => Some(self)`.

## Constructors, timestamps, parsing

- Low-level: `from_naive_utc_and_offset` (const, canonical). `from_utc`/`from_local` are deprecated.
- `DateTime<Utc>` timestamp constructors are all `const`: `from_timestamp_secs`, `from_timestamp(secs, nsecs)`, `from_timestamp_millis`/`from_timestamp_micros` (→ `Option`), and `from_timestamp_nanos` (infallible — every `i64` nanosecond value is in range). Associated consts: `MIN_UTC`, `MAX_UTC`, `UNIX_EPOCH`; `UNIX_EPOCH_DAY` (`pub(crate)`, `719_163`) backs `timestamp()`/`from_timestamp()`.
- `timestamp_nanos_opt` carries a negative-timestamp underflow workaround (`((ts+1)*1e9)+(ns-1e9)`, chrono#1289); `timestamp_nanos()` is the deprecated panicking form. Subsecond accessors can exceed their nominal maxima during a leap second (`secs % 60 == 59`).
- Parsing lives on `DateTime<FixedOffset>` and **requires an offset in the input**: `parse_from_rfc2822`, `parse_from_rfc3339`, `parse_from_str`, `parse_and_remainder`. `FromStr` for `DateTime<Utc>`/`DateTime<Local>` parses a relaxed RFC 3339 (space or `T` separator) as `FixedOffset`, then converts.

## serde (`serde.rs`, feature `serde`)

The default `Serialize` writes an RFC 3339 string (`SecondsFormat::AutoSi`, `use_z = true`); `Deserialize` is provided for `FixedOffset`/`Utc`/`Local` through a shared `DateTimeVisitor`. The integer-timestamp adapters are `pub mod`s re-exported at `crate::serde`, for use with `#[serde(with = "...")]`: `ts_seconds`, `ts_milliseconds`, `ts_microseconds`, `ts_nanoseconds`, and their `_option` variants (over `Option<DateTime<Utc>>`). All operate on `DateTime<Utc>`. `ts_nanoseconds` serialize **errors** on out-of-range values rather than panicking.

## Tests and cross-directory dependencies

`tests.rs` and the `#[cfg(test)]` module inside `serde.rs` are **upstream-style**: they use `assert!`/`assert_eq!`/`unwrap()`/`#[should_panic]` and a few local `#[allow(...)]`s, and are deliberately not converted to the strict `Result<(), TestFailure>` + `ensure*` vocabulary — keep that style so upstream merges stay clean. `DstTester` (a synthetic +9/+8 DST `TimeZone` with ambiguous/none windows) is the main fixture for exercising DST behavior in `Days`/`Months` arithmetic.

This directory consumes the other three: UTC storage and offset/arithmetic math bottom out in `../naive` (`NaiveDateTime::{MIN, MAX, checked_add_offset, overflowing_add_offset, …}`); timezone resolution comes from `../offset` (`TimeZone`, `Offset::fix`, `LocalResult`); formatting and parsing delegate to `../format` (`Parsed`, `parse*`, `write_rfc2822`/`write_rfc3339`, `DelayedFormat`, `StrftimeItems`). See those directories' `AGENTS.md` for their internals rather than duplicating them here.
