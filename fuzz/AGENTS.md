# fuzz — cargo-fuzz / libFuzzer targets

`chrono-fuzz` (edition 2018, version `0.0.0`, `publish = false`) is a cargo-fuzz crate that path-depends on `chrono` and on `libfuzzer-sys`. It declares its own `[workspace] members = ["."]` so it detaches from the parent crate and does not get pulled into any surrounding build.

Two targets under `fuzz_targets/`, both of which discard results — they exist to surface panics or UB in the parsers, not to assert:

- `fuzz_reader` — `|data: &[u8]|`; when the bytes are valid UTF-8, drives `DateTime::parse_from_rfc2822` and `DateTime::parse_from_rfc3339`.
- `fuzz_format` — `|data: (String, String)|`; drives `DateTime::parse_from_str(&input, &format)` over an arbitrary input/format-string pair.

Run with cargo-fuzz (nightly): `cargo fuzz run fuzz_reader` or `cargo fuzz run fuzz_format`. `README.md` documents the `cargo install cargo-fuzz` bootstrap; the repository also defines a `just fuzz` recipe. The parser entry points these targets exercise live in `../src/format` and `../src/datetime` — see those directories' `AGENTS.md`.

Library layer — kept mergeable with upstream `chronotope/chrono`; keep the targets minimal and do not restyle to strict conventions.
