# tests — cross-checked integration tests and tzdata fixtures

Each Cargo integration-test target here is narrowly `#[cfg]`-gated so it only compiles where it can actually run, and two of them validate chrono against an external oracle rather than hardcoded expectations.

## Integration-test targets

- `dateutils.rs` — `#![cfg(all(unix, feature = "clock", feature = "std"))]`. Cross-checks `Local` against the system `date` binary (`DATE_PATH` = `/usr/bin/date`, or `/opt/freeware/bin/date` on AIX). `try_verify_against_date_command` spawns threads that walk every hour across a set of sampled years and tolerates DST ambiguity — a `MappedLocalTime::Ambiguous` result accepts either wall-clock string, `None` expects empty output. `try_verify_against_date_command_format` (Linux-only, forced `LANG=c`/`LC_ALL=c`) asserts a long `%`-spec string matches `date` byte-for-byte; its doc comments record which specifiers are excluded because they are locale-dependent. Both skip gracefully when the `date` binary is absent.
- `wasm.rs` — `#![cfg(all(target_family = "wasm", feature = "wasmbind", feature = "clock", not(any(target_os = "emscripten", target_os = "wasi"))))]`. `#[wasm_bindgen_test]` cases comparing `Utc::now`/`Local::now` and `js_sys::Date` round-trips against the `TZ`/`NOW` env vars the runner sets (the file header documents the `wasm-pack test --node` invocation). The `TZ` match is a closed set — an unrecognized value panics deliberately.
- `win_bindings.rs` — regenerates `src/offset/local/win_bindings.rs` via `windows_bindgen::bindgen` and asserts (line-by-line, CRLF-normalized) that the checked-in file is byte-identical. This is the guard that keeps that generated file honest: **regenerate it, never hand-edit it**, and the filter list here (`GetTimeZoneInformationForYear`, `SystemTimeToFileTime`, `SystemTimeToTzSpecificLocalTime`, `TzSpecificLocalTimeToSystemTime`) is the source of truth for what the file must contain.

## tzdata fixtures (not test code)

`tests/android/tzdata` and `tests/ohos/tzdata` are **binary ZoneInfoDb files**, not integration tests. No `tests/*.rs` references them. They are opened by relative path (`./tests/android/tzdata`, `./tests/ohos/tzdata`) from the `#[cfg(test)]` unit tests inside `../src/offset/local/tz_data.rs`, so those unit tests only pass when the working directory is the crate root. See `../src/offset/AGENTS.md` for the Android/OpenHarmony tzdata parser they cover.

## Style

Library layer — kept mergeable with upstream `chronotope/chrono`. `#[test]`/`#[wasm_bindgen_test]` functions returning `()`, `assert_eq!`/`assert!`, `unwrap()`, and the module-level `#[allow(dead_code)]` on the cfg-conditional `DATE_PATH` are intentional upstream idioms; do not convert them to the strict `Result<(), TestFailure>` + `ensure*` vocabulary.
