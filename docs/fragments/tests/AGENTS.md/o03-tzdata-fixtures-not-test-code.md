## tzdata fixtures (not test code)

`tests/android/tzdata` and `tests/ohos/tzdata` are **binary ZoneInfoDb files**, not integration tests. No `tests/*.rs` references them. They are opened by relative path (`./tests/android/tzdata`, `./tests/ohos/tzdata`) from the `#[cfg(test)]` unit tests inside `../src/offset/local/tz_data.rs`, so those unit tests only pass when the working directory is the crate root. See `../src/offset/AGENTS.md` for the Android/OpenHarmony tzdata parser they cover.
