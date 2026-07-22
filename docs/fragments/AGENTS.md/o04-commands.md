## Commands

`just` is the canonical entry point; each recipe is a thin dispatch into the installed `template` binary. `just` with no argument lists every recipe.

- `just ci` — the full local CI mirror in CI order. `just precommit` — the pre-commit gate (also stages regenerated metrics and badges).
- `just fmt` (nightly rustfmt), `just check` (type-check), `just lint` (strict Clippy), `just doc` (rustdoc).
- `just test` runs nextest unit and integration tests **only** — it does not run doctests. `just test-doc` runs doctests. Use `just test-all` for a full test handoff (nextest + doctests).
- `just hack` runs the cargo-hack feature matrix, `just audit` runs cargo-audit + cargo-deny, `just cq` runs the cyclomatic/duplication/tokei code-quality checks, and `just coverage` runs cargo-llvm-cov.
- Scoping and passthrough: everyday code commands forward their arguments verbatim; a leading PATH scopes the run to that path, and anything after `--` is forwarded to the underlying tool. Examples: `just lint src`, `just lint src -- --fix`.
- Run a subset of tests by forwarding a nextest filter after `--`: `just test -- <name-substring>` (e.g. `just test -- NaiveDate`). For a single doctest, `just test-doc -- <filter>`.

`Cargo.lock` is intentionally not committed (see `.gitignore`) — this is a library crate, and feature combinations are validated with `just hack`.
