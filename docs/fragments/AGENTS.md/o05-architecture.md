## Architecture

The public API is re-exported from `src/lib.rs`, which also owns feature gating and the `prelude`. The modules underneath:

- `time_delta.rs` — `TimeDelta` (exported also as `Duration`), the signed, "accurate" span type measured in seconds + nanoseconds; `OutOfRangeError` for `std::time::Duration` conversions.
- `datetime/` — `DateTime<Tz>`, the timezone-aware instant, plus feature-gated `serde` support.
- `naive/` — timezone-naive types: `date/` (`NaiveDate`), `time/` (`NaiveTime`), `datetime/` (`NaiveDateTime`), and `isoweek.rs` (`IsoWeek`). `internals.rs` holds the bit-packed date representation and the year-flag / leap-year tables that `NaiveDate` is built on; most calendar arithmetic bottoms out here.
- `offset/` — the timezone abstraction. `mod.rs` defines the `TimeZone`/`Offset` traits and `MappedLocalTime`; `utc.rs` is `Utc`, `fixed.rs` is `FixedOffset`, and `local/` is the OS timezone integration. `local/` splits into `unix.rs`, `windows.rs` + `win_bindings.rs`, and `tz_info/` (`parser.rs`, `rule.rs`, `timezone.rs`) plus `tz_data.rs` — a TZif parser forked from the `tz-rs` crate. Since ~4.20 chrono reads the system timezone database directly instead of calling `localtime_r` (the RUSTSEC-2020-0159 fix); preserve that invariant.
- `format/` — parsing and formatting. `strftime.rs` lexes a format string into `Item`s (`StrftimeItems`), `formatting.rs` renders, `parse.rs` + `scan.rs` parse, `parsed.rs` is `Parsed` (the accumulator a parse fills and then resolves into concrete date/time types), and `locales.rs` backs the `unstable-locales` `_localized` methods.
- Field access is through the `Datelike`/`Timelike` traits in `traits.rs`. `weekday.rs` (`Weekday`), `weekday_set.rs` (`WeekdaySet`), and `month.rs` (`Month`, `Months`) are the supporting enums; `round.rs` adds rounding/truncation; `date.rs` is the deprecated `Date<Tz>`.
- `serde`/`rkyv`/`defmt`/`arbitrary` integration is feature-gated at the crate root.

Integration tests in `tests/`: `dateutils.rs` cross-checks formatting/parsing against the system `date` command on Unix; `wasm.rs` targets wasm-bindgen; `win_bindings.rs` asserts the checked-in `src/offset/local/win_bindings.rs` matches what `windows-bindgen` would generate — regenerate that file rather than hand-editing it. `bench/` holds benchmarks behind the internal `__internal_bench` feature and `fuzz/` holds fuzz targets.

Feature flags: defaults are `clock`, `std`, `oldtime`, `wasmbind`; `std` is a superset of `alloc` and `clock` a superset of `now`; `oldtime` is now a no-op. The `rkyv`, `rkyv-16`, `rkyv-32`, and `rkyv-64` features are mutually exclusive.
