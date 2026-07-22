# AGENTS.md

This file provides guidance to coding agents when working with code in this repository.

## What this repository is

`strict-chrono` (remote `github.com/strict-rs/strict-chrono`) is a hard fork of upstream [`chronotope/chrono`](https://github.com/chronotope/chrono). The crate is still named and published as `chrono` (`Cargo.toml` `name = "chrono"`, `[lib] name = "chrono"`, version tracks upstream — currently `0.4.45`). It provides timezone-aware date and time handling on the proleptic Gregorian calendar: `DateTime<Tz>` is timezone-aware by default with separate timezone-naive types, operations that can produce an invalid or ambiguous result return `Option` or `MappedLocalTime`, and parsing/formatting uses an `strftime`-inspired syntax. Timezone data is not bundled; `Local` reads the OS timezone.

## Two layers — do not confuse them

This repository is upstream chrono's **library source** with the strict ecosystem's **tooling** layered around it. The two follow different rules, and the most common way to do the wrong thing here is to apply one layer's conventions to the other.

- **Library source** — `src/`, `tests/`, `bench/`, `fuzz/` — is upstream chrono, kept mergeable with `chronotope/chrono`. It deliberately keeps upstream conventions: `edition = "2021"`, an MSRV (`Cargo.toml` `rust-version = "1.62.0"`), the crate-level lint attributes in `src/lib.rs` (`#![warn(unreachable_pub)]`, `no_std` gating, `deny(warnings)` inside doctests), and upstream test style (`#[test]` functions returning `()`, `assert_eq!`, `unwrap()`, `expect()`). Do **not** rewrite this source to the strict-family source policy (edition 2024, panic-free `strict_test_support` tests, no `#[allow]`, enum-only errors). Preserve upstream idioms and keep diffs minimal so upstream releases stay mergeable; fix a real bug at its root, but don't restyle upstream code to match strict conventions.
- **Strict tooling** is everything around the source: the `justfile` (canonical command surface, dispatching to the installed `template` binary), `xtask/`, `rust-toolchain.toml` (rolling stable), and the lint/format/supply-chain config whose key ownership is declared in `template.config.toml` (`clippy.toml`, `rustfmt.toml`, `deny.toml`, and the generated `template-clippy.toml`/`template-lint.toml`). `xtask/` is the one directory that *is* strict-family code — it is governed by its own `xtask/AGENTS.md`, not by chrono's conventions.

Generated-doc caution: `template.config.toml` `[guidance]` declares `AGENTS.md`, `README.md`, and `xtask/AGENTS.md` as `just gen-md` targets, and a precommit gate re-runs `just gen-md` whenever any `AGENTS.md`/`README.md`/`CLAUDE.md`/fragment changes. `xtask/AGENTS.md` and `xtask/README.md` are already generated (they carry a `Do not edit; generated file.` marker). This root `AGENTS.md` is currently hand-maintained — treat it as the source of truth until its content is moved into `just gen-md` fragments.

## Commands

`just` is the canonical entry point; each recipe is a thin dispatch into the installed `template` binary. `just` with no argument lists every recipe.

- `just ci` — the full local CI mirror in CI order. `just precommit` — the pre-commit gate (also stages regenerated metrics and badges).
- `just fmt` (nightly rustfmt), `just check` (type-check), `just lint` (strict Clippy), `just doc` (rustdoc).
- `just test` runs nextest unit and integration tests **only** — it does not run doctests. `just test-doc` runs doctests. Use `just test-all` for a full test handoff (nextest + doctests).
- `just hack` runs the cargo-hack feature matrix, `just audit` runs cargo-audit + cargo-deny, `just cq` runs the cyclomatic/duplication/tokei code-quality checks, and `just coverage` runs cargo-llvm-cov.
- Scoping and passthrough: everyday code commands forward their arguments verbatim; a leading PATH scopes the run to that path, and anything after `--` is forwarded to the underlying tool. Examples: `just lint src`, `just lint src -- --fix`.
- Run a subset of tests by forwarding a nextest filter after `--`: `just test -- <name-substring>` (e.g. `just test -- NaiveDate`). For a single doctest, `just test-doc -- <filter>`.

`Cargo.lock` is intentionally not committed (see `.gitignore`) — this is a library crate, and feature combinations are validated with `just hack`.

## Architecture

The public API is re-exported from `src/lib.rs`, which also owns feature gating and the `prelude`. The modules underneath:

- `time_delta.rs` — `TimeDelta` (exported also as `Duration`), the signed, "accurate" span type measured in seconds + nanoseconds; `OutOfRangeError` for `std::time::Duration` conversions.
- `datetime/` — `DateTime<Tz>`, the timezone-aware instant, plus feature-gated `serde` support.
- `naive/` — timezone-naive types: `date/` (`NaiveDate`), `time/` (`NaiveTime`), `datetime/` (`NaiveDateTime`), and `isoweek.rs` (`IsoWeek`). `internals.rs` holds the bit-packed date representation and the year-flag / leap-year tables that `NaiveDate` is built on; most calendar arithmetic bottoms out here.
- `offset/` — the timezone abstraction. `mod.rs` defines the `TimeZone`/`Offset` traits and `MappedLocalTime`; `utc.rs` is `Utc`, `fixed.rs` is `FixedOffset`, and `local/` is the OS timezone integration. `local/` splits into `unix.rs`, `windows.rs` + `win_bindings.rs`, and `tz_info/` (`parser.rs`, `rule.rs`, `timezone.rs`) plus `tz_data.rs` — a TZif parser forked from the `tz-rs` crate. Since ~4.20 chrono reads the system timezone database directly instead of calling `localtime_r` (the RUSTSEC-2020-0159 fix); preserve that invariant.
- `format/` — parsing and formatting. `strftime.rs` lexes a format string into `Item`s (`StrftimeItems`), `formatting.rs` renders, `parse.rs` + `scan.rs` parse, `parsed.rs` is `Parsed` (the accumulator a parse fills and then resolves into concrete date/time types), and `locales.rs` backs the `unstable-locales` `_localized` methods.
- Field access is through the `Datelike`/`Timelike` traits in `traits.rs`. `weekday.rs` (`Weekday`), `weekday_set.rs` (`WeekdaySet`), and `month.rs` (`Month`, `Months`) are the supporting enums; `round.rs` adds rounding/truncation; `date.rs` is the deprecated `Date<Tz>`.
- `serde`/`rkyv`/`defmt`/`arbitrary` integration is feature-gated at the crate root.

Integration tests in `tests/`: `dateutils.rs` cross-checks formatting/parsing against the system `date` command on Unix; `wasm.rs` targets wasm-bindgen; `win_bindings.rs` asserts the checked-in `src/offset/local/win_bindings.rs` matches what `windows-bindgen` would generate — regenerate that file rather than hand-editing it. `bench/` holds benchmarks behind the internal `__internal_bench` feature and `fuzz/` holds fuzz targets.

Feature flags: defaults are `clock`, `std`, `oldtime`, `wasmbind`; `std` is a superset of `alloc` and `clock` a superset of `now`; `oldtime` is now a no-op. The `rkyv`, `rkyv-16`, `rkyv-32`, and `rkyv-64` features are mutually exclusive.

## Commit messages

- Subject: `type(scope): structural imperative description` — conventional-commit style, scope **required**, `!` appended for breaking changes. **Never `chore`** — pick the precise type (`feat`, `fix`, `refactor`, `perf`, `build`, `ci`, `docs`, `test`, `style`, `revert`, …).
- Headline the substance: the subject names the most significant behavior or API change; renames, moves, lockfile bumps, and generated artifacts are fallout, never the headline when real behavior also changed.
- Body: 1–5 sections sized to the commit. Each section starts with a plain-text header line (no `#`, no bold), followed by 3–5 imperative bullets describing structural changes; exactly one blank line between sections; fallout goes in the last section.
- Pass the message via HEREDOC to `git commit -m` so blank lines survive shell quoting.
