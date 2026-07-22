# bench — Criterion benchmarks (standalone `benches` crate)

This is its own package (`name = "benches"`, edition 2021, `publish = false`), **not** a member of the chrono crate — the repo root declares no `[workspace]`, so `benches` path-depends on `..` and stands alone. The split is deliberate: keeping Criterion and its dependency tree out of chrono's own dev-dependencies stops them from raising chrono's MSRV (chrono#1104). `src/lib.rs` is an empty placeholder that exists only to make the crate valid; the real content is the two `harness = false` Criterion targets under `benches/`.

## What the targets need

`benches` depends on `chrono = { path = "..", features = ["__internal_bench", "serde"] }`, and re-exposes an `unstable-locales` passthrough feature.

- `__internal_bench` unlocks `chrono::__BenchYearFlags` — the otherwise-private `naive::internals` year-flag type that `bench_year_flags_from_year` exercises. Without that feature the target does not compile. See `../src/naive/AGENTS.md` for what that type represents.
- `serde` is required by `benches/serde.rs`.
- `bench_get_local_time`/`bench_format`/`benches_delayed_format` use `Local::now`, so they lean on chrono's default `clock`/`std`.

## The two targets

- `benches/chrono.rs` — date construction, RFC 2822/3339 and `FromStr` parsing, RFC 2822/3339 formatting, `num_days_from_ce` (benchmarked against a local reimplementation `num_days_from_ce_alt` to keep the fast path honest), `Local::now`, `StrftimeItems` lexing, the `DelayedFormat` `Display`-vs-`write_to` render paths, `NaiveDate::checked_add_signed`, and `with_hour`. Under `unstable-locales` it adds `bench_parse_strftime_localized` in a second `criterion_group!`, and the `criterion_main!` line is `cfg`-selected to include that group.
- `benches/serde.rs` — `NaiveDateTime` serialized through `serde_json`, string form vs `to_writer`.

## Running and style

Run from this directory: `cargo bench` (add `--features unstable-locales` for the localized group). There is no `just` recipe for benchmarks.

Library layer — kept mergeable with upstream `chronotope/chrono`. These files use `unwrap()`, `assert_eq!`, and `criterion::black_box` by design; do not restyle them to strict test/lint conventions.
