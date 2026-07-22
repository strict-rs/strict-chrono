## The two targets

- `benches/chrono.rs` — date construction, RFC 2822/3339 and `FromStr` parsing, RFC 2822/3339 formatting, `num_days_from_ce` (benchmarked against a local reimplementation `num_days_from_ce_alt` to keep the fast path honest), `Local::now`, `StrftimeItems` lexing, the `DelayedFormat` `Display`-vs-`write_to` render paths, `NaiveDate::checked_add_signed`, and `with_hour`. Under `unstable-locales` it adds `bench_parse_strftime_localized` in a second `criterion_group!`, and the `criterion_main!` line is `cfg`-selected to include that group.
- `benches/serde.rs` — `NaiveDateTime` serialized through `serde_json`, string form vs `to_writer`.
