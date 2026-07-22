<!-- Do not edit; generated file. -->

# ci/core-test — `#![no_std]` compile-smoke crate

`core-test` (edition 2018) is a tiny standalone crate whose only job is to fail the build if chrono ever accidentally pulls in `std` on a `no_std` target. It path-depends on `chrono` with `default-features = false, features = ["serde"]`, and exposes an `alloc` feature that forwards to `chrono/alloc`.

`src/lib.rs` is `#![no_std]` and holds a single `create_time()` that builds `Utc.with_ymd_and_hms(2019, 1, 1, 0, 0, 0)` — enough to link the default + `serde` (+ optional `alloc`) surface without `std`. It asserts nothing; success *is* the crate compiling against `core`.

As the `ci/` path implies, this is driven from CI, not from the `just` command surface. It is upstream chrono's CI helper, kept mergeable — keep it minimal and `no_std`; do not add `std`-dependent code or convert it to strict conventions.
