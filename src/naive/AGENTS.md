<!-- Do not edit; generated file. -->

# src/naive — timezone-naive date and time types

Owns the timezone-agnostic building blocks the rest of the crate is composed from: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `IsoWeek`, and the supporting `NaiveWeek` / `Days` newtypes. `mod.rs` re-exports them and declares the `serde` submodule; the type impls live in `date/`, `time/`, `datetime/`, `isoweek.rs`, and the bit-packing machinery in `internals.rs`.

## The packed representations (the load-bearing detail)

Every naive type is a tightly bit-packed integer, and `internals.rs` owns the shared machinery. Understand this before touching anything in `date/` or `isoweek.rs`:

- `NaiveDate` is a single `NonZeroI32` named `yof`, laid out as `(year << 13) | (ordinal << 4) | flags` — year, day-of-year (1..=366), then a 4-bit `YearFlags`. So a `NaiveDate` is 4 bytes and, thanks to the `NonZero` niche, `Option<NaiveDate>` is also 4 bytes. The masks and bounds are the consts near the bottom of `date/mod.rs`: `ORDINAL_MASK`, `LEAP_YEAR_MASK` (`0b1000`), `OL_MASK`, `MAX_OL` (`366 << 4`), `WEEKDAY_FLAGS_MASK` (`0b111`), `YEAR_FLAGS_MASK`, and `MIN_YEAR`/`MAX_YEAR` (one inside the capacity of `i32 >> 13`).
- `YearFlags(u8)` (in `internals.rs`) is `LWWW`: bit 3 is the leap-year flag (`1` = common year, chosen so ordinal validation is a single comparison), bits 0–2 hold the weekday of the last day of the *preceding* year (for fast weekday math). The `A`..`G` / `AG`..`GF` dominical-letter consts and the 400-entry `YEAR_TO_FLAGS` table encode all 14 year classes.
- `Mdf(u32)` (month-day-flags, `(month << 9) | (day << 4) | flags`) is the intermediate for calendar-date construction, and it **validates late**: an impossible date like Feb 30 can be represented, and validity is only decided at the `MDL_TO_OL` / `OL_TO_MDL` lookup-table conversion to/from the ordinal form (~1.5 KB of tables). Preserve that "represent first, validate at the table" contract — it is what makes construction fast.
- `IsoWeek` (`isoweek.rs`) packs `(year << 10) | (week << 4) | flags` into an `i32` and deliberately allows a wider year range than `NaiveDate`, because the ISO-week year of the first/last representable days differs from the calendar year.

## Range, sentinels, and the out-of-range buffer

`NaiveDate` spans about ±262,143 years. `MIN_YEAR`/`MAX_YEAR` sit one year inside the raw capacity on purpose: the extra headroom holds `NaiveDate::BEFORE_MIN` and `AFTER_MAX`, which `NaiveDateTime::overflowing_add/sub_offset` fall back to. This is the naive-layer half of the `DateTime::overflowing_naive_local` trick — an out-of-range local value can exist transiently but must never be handed to a user.

## Leap seconds live in `NaiveTime`

`NaiveTime { secs: u32, frac: u32 }` stores seconds-from-midnight (`0..86_400`) and a nanosecond `frac`. A leap second is `frac >= 1_000_000_000`, accepted only when `secs % 60 == 59`; `frac` may reach `2_000_000_000`. `second()` returns 59 during a leap second (formatting reconstructs `60`). Arithmetic wraps within a day and *ignores* whole days: `overflowing_add_signed`/`overflowing_sub_signed` return `(NaiveTime, i64)` where the `i64` is the day-carry in seconds, and `overflowing_add/sub_offset` return `(NaiveTime, i32)` day-carry — this is how `NaiveDateTime` and `DateTime` push the carry into the date while preserving leap seconds.

## `NaiveDateTime` and the rest of the module

`NaiveDateTime { date, time }` composes the two and delegates `Datelike`/`Timelike` through to them; its `checked_add/sub_offset` are the leap-second-preserving offset shifts, and its many `timestamp*` methods are deprecated in favour of `.and_utc()` on the resulting `DateTime<Utc>`. `mod.rs` also defines `NaiveWeek` (a `NaiveDate` plus a start `Weekday`, with `first_day`/`last_day`/`checked_*`/`days()` range) and the `Days(u64)` newtype.

serde: `naive::serde` re-exports `ts_seconds`/`ts_milliseconds`/`ts_microseconds`/`ts_nanoseconds` (and `_option` variants) for `NaiveDateTime`. These are **distinct** from the identically named adapters under `crate::serde`, which target `DateTime<Utc>`; don't conflate them.

## Conventions and one strict-vs-upstream tension

Tests across `date/`, `time/`, `datetime/` are upstream-style (`assert!`/`unwrap()`), and `internals.rs` / `date/tests.rs` pin the bit-packing against the 14-entry `YEAR_FLAGS` dominical-letter table — preserve that shape. One thing to flag for any strict-policy conversion: `NaiveDate::from_yof` uses `unsafe { NonZeroI32::new_unchecked(yof) }`. `unsafe` is forbidden workspace-wide in the strict family, so removing it means refactoring the constructor (e.g. a checked `NonZeroI32::new`), not adding an allowance. (The platform `../offset/local` code also contains `unsafe` for FFI and zero-copy string views.)
