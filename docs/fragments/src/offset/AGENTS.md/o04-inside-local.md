## Inside `local/`

`Local` selects one `inner` implementation by target:

- `unix.rs` — a thread-local `Cache` holding a parsed `tz_info::TimeZone`. The cache is keyed by a `Source` (either the mtime of `/etc/localtime` or a hash of the `TZ` variable) and re-checked at most once per second, so repeated conversions are cheap while a changed system zone is still picked up quickly. It reads `/etc/localtime` or `$TZ`, with an `iana-time-zone` + `TZDB_LOCATION` fallback.
- `windows.rs` — builds a `TzInfo` per year from `GetTimeZoneInformationForYear` and resolves it with the shared `lookup_with_dst_transitions` helper (defined in `mod.rs`, alongside the `Transition` type). It deliberately does *not* use the OS `*LocalTimeToSystemTime` calls, so it can control gap/fold handling itself; a test cross-checks it against `TzSpecificLocalTimeToSystemTime`.
- A wasm (`js_sys::Date`) path and a plain-UTC fallback round out the `cfg` matrix.

`local/tz_info/` is a TZif (compiled zoneinfo) and POSIX-`TZ`-string parser forked from the `tz-rs` crate — this is the code that lets the crate read the system zone database directly instead of calling `localtime_r` (the RUSTSEC-2020-0159 fix). It is self-contained (its own `Error` enum, `#![deny(missing_docs)]`): `parser.rs` (TZif v1/v2/v3 binary format), `rule.rs` (`TransitionRule`/`AlternateTime`, DST rule days, proleptic-Gregorian math), and `timezone.rs` (`TimeZone`/`TimeZoneRef`, transitions, leap seconds, `LocalTimeType`). `local/tz_data.rs` handles the bundled `tzdata` blob format used on Android and OpenHarmony.
