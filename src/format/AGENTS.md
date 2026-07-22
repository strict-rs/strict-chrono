<!-- Do not edit; generated file. -->

# src/format — the shared parse + format engine

Every `to_string`/`format`/`parse*` path in the crate routes through here — `DateTime`, the `Naive*` types, and the offset types own no parsing or formatting logic of their own. Keep this module's public `Item`/`Numeric`/`Fixed`/`ParseError` surface stable; it is the shared contract those types depend on.

## Pipeline and the shared `Item` vocabulary

One enum, `Item<'a>` (in `mod.rs`), is the intermediate representation for both directions — which is exactly what lets a single format string round-trip:

format string → `StrftimeItems` (lexer, `strftime.rs`) → `Iterator<Item = Item>` → { render via `DelayedFormat` (`formatting.rs`) | parse via `parse` (`parse.rs`), filling a `Parsed` (`parsed.rs`) } → `Parsed::to_*` resolves to `NaiveDate` / `NaiveTime` / `NaiveDateTime` / `DateTime` / `FixedOffset`.

`Item` variants: `Literal` / `OwnedLiteral` (alloc) / `Space` / `OwnedSpace` (alloc) / `Numeric(Numeric, Pad)` / `Fixed(Fixed)` / `Error`. `Numeric` and `Fixed` are `#[non_exhaustive]` enums naming every field and format primitive (`Year`…`Timestamp`; `ShortMonthName`…`RFC3339`). Consumers build items through `StrftimeItems` or as `&[Item]` slices.

## File map

- `mod.rs` — the vocabulary (`Item`, `Numeric`, `Fixed`, `Pad`, `OffsetFormat`/`OffsetPrecision`/`Colons`, `SecondsFormat`) and the error model (`ParseError`, `ParseErrorKind`, `ParseResult`, plus the internal construction consts `OUT_OF_RANGE`/`IMPOSSIBLE`/`NOT_ENOUGH`/`INVALID`/`TOO_SHORT`/`TOO_LONG`/`BAD_FORMAT` — of these only `OUT_OF_RANGE` and `TOO_LONG` are `pub(crate)`, the rest are module-private). Holds the crate re-export surface and the `FromStr` impls for `Weekday`/`Month` (they live here because they need the private `scan` code).
- `strftime.rs` — `StrftimeItems`, the `%`-format-string lexer; `impl Iterator<Item = Item>`. `new` turns an invalid specifier into `Item::Error`; `new_lenient` turns it into a `Literal` of the raw text; `new_with_locale` (unstable-locales) recurses over locale format strings.
- `formatting.rs` — rendering: `DelayedFormat<I>` (the lazy value returned by `.format()`; its `Display` renders to a `String` then applies `f.pad(..)` so width/fill/align cover the whole output), the `format_numeric`/`format_fixed` dispatch, `OffsetFormat::format`, and `write_rfc2822`/`write_rfc3339`/`write_hundreds`.
- `parse.rs` — the engine: `parse` (requires the whole string consumed, else `TOO_LONG`), `parse_and_remainder` (returns the unparsed tail), plus the dedicated `parse_rfc2822`, strict `parse_rfc3339`, and looser `parse_rfc3339_relaxed` grammars.
- `parsed.rs` — `Parsed`, the field accumulator and its consistency-checked resolution.
- `scan.rs` — the low-level byte scanners `parse.rs` is built on (`number`, `nanosecond`/`nanosecond_fixed`, `short_or_long_month0`/`weekday`, `timezone_offset`/`timezone_offset_2822`, `comment_2822`, …); the `scan` module is `pub(crate)` while most of its functions are `pub(super)`.
- `locales.rs` — `cfg`-selected name tables: `pure-rust-locales` under `unstable-locales`, hardcoded English (with `Locale` as a ZST) otherwise.

## `Parsed` resolution — the consistency contract

`Parsed` has 21 all-`Option` fields (year plus its div/mod-100 and ISO variants, quarter, month, week_from_sun/mon, isoweek, weekday, ordinal, day, hour_div_12/hour_mod_12, minute, second, nanosecond, timestamp, offset). Every `set_*` funnels through `set_if_consistent`: re-setting an already-set field to a **different** value returns `IMPOSSIBLE` (setting the same value is fine), and out-of-range values return `OUT_OF_RANGE`. That is how contradictory format inputs are rejected up front.

The resolvers (`to_naive_date`, `to_naive_time`, `to_naive_datetime_with_offset`, `to_fixed_offset`, `to_datetime`, `to_datetime_with_timezone`) do more than read fields: each picks a construction path (e.g. year+month+day, year+ordinal, year+week+weekday, isoyear+isoweek+weekday), builds the value, then **re-derives the other representations and requires every set field to match**, so an internally inconsistent parse (a weekday that disagrees with the date, a timestamp that disagrees with the components) fails rather than silently resolving. Two-digit years use a century heuristic (`+2000` if `< 70`, else `+1900`; RFC 2822 uses `< 50`).

## Invariants and gotchas to preserve

- **Leap seconds are represented uniformly** as second `60` / nanosecond `≥ 1_000_000_000`: `set_second` accepts 0–60, resolution folds `60` into `59` + an extra 1e9 nanos, formatting reconstructs `60` from `second + nanosecond / 1e9`, and timestamp reconciliation allows a ±1 s discrepancy. Do not "tidy" these bounds to 0–59 / 0–999_999_999.
- **Parsing ignores `Pad`** — the padding modifier only affects formatting. When parsing, `Numeric` items trim leading whitespace and consume up to the spec's intrinsic width, which is what lets packed forms like `%H%M%S` parse.
- **Offset minute-rounding is clamped** in `OffsetFormat::format`: `((off + 30) / 60).min(24 * 60 - 1)`, so an offset in the final half-minute (e.g. 23:59:59) never rounds up to an invalid `+24:00`. This is a fork fix with a dedicated regression test (`test_offset_minute_rounding_no_24h_rollover`) — do not regress it.
- **Internal, non-stable items** hide behind the opaque `InternalNumeric`/`InternalFixed` wrappers so new internal-only items can be added without a breaking change. `InternalFixed`'s `TimezoneOffsetPermissive` (the `%#z` target) is **parse-only** — formatting it yields a `fmt::Error` rather than panicking (the mod.rs doc-comment says "panics", but `format_fixed` has no arm for it so it falls through to `Err`, and `test_parse_only_timezone_offset_permissive_no_panic` pins the no-panic behavior for chrono#1139); the `Nanosecond{3,6,9}NoDot` internals print/parse fixed digits without the leading dot. `Numeric::Internal` is currently uninhabited and unreachable.
- **`Fixed::TimezoneName` (`%Z`) is offset-only** — chrono carries no zone abbreviations, so formatting prints the stored offset string and parsing skips the name without storing anything.

## cfg gating and consumers

`alloc` gates `DelayedFormat`, the `Owned*` item variants, the deprecated free `format`/`format_item`, and `write_rfc2822`/`write_rfc3339` (`write_rfc3339` is also compiled under `serde`). `unstable-locales` gates `Locale`, `new_with_locale`, and the `format_localized*` paths. The `Error` impl on `ParseError` is under `core-error`/`std`.

Consumers import from here rather than reimplementing: `datetime` and the `naive/*` types build `StrftimeItems` or item slices and hand them to `DelayedFormat` (format) or `parse`/`Parsed` (parse), relying on `Parsed::to_*`; `offset` uses `parse`/`Parsed`/`StrftimeItems`, and `offset/fixed.rs` uses `scan` + `OUT_OF_RANGE` directly.
