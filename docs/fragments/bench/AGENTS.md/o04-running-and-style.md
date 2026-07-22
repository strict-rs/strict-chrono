## Running and style

Run from this directory: `cargo bench` (add `--features unstable-locales` for the localized group). There is no `just` recipe for benchmarks.

Library layer — kept mergeable with upstream `chronotope/chrono`. These files use `unwrap()`, `assert_eq!`, and `criterion::black_box` by design; do not restyle them to strict test/lint conventions.
