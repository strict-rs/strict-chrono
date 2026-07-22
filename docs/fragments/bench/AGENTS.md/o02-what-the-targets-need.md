## What the targets need

`benches` depends on `chrono = { path = "..", features = ["__internal_bench", "serde"] }`, and re-exposes an `unstable-locales` passthrough feature.

- `__internal_bench` unlocks `chrono::__BenchYearFlags` — the otherwise-private `naive::internals` year-flag type that `bench_year_flags_from_year` exercises. Without that feature the target does not compile. See `../src/naive/AGENTS.md` for what that type represents.
- `serde` is required by `benches/serde.rs`.
- `bench_get_local_time`/`bench_format`/`benches_delayed_format` use `Local::now`, so they lean on chrono's default `clock`/`std`.
