## Consumers and conventions

`mod.rs`'s `TimeZone` trait is the primary source of `DateTime` constructors (`with_ymd_and_hms`, `timestamp_opt`, `from_local_datetime`, `from_utc_datetime`, …); several `Date`-based methods (`ymd`, `yo`, `isoywd`, `datetime_from_str`) are deprecated. Tests here are upstream-style, and the `tz_info` submodule carries extensive unit tests over real TZif byte fixtures — keep those as the contract when changing the parser.
