# bench — Criterion benchmarks (standalone `benches` crate)

This is its own package (`name = "benches"`, edition 2021, `publish = false`), **not** a member of the chrono crate — the repo root declares no `[workspace]`, so `benches` path-depends on `..` and stands alone. The split is deliberate: keeping Criterion and its dependency tree out of chrono's own dev-dependencies stops them from raising chrono's MSRV (chrono#1104). `src/lib.rs` is an empty placeholder that exists only to make the crate valid; the real content is the two `harness = false` Criterion targets under `benches/`.
