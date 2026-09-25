//! `chrono-rkyv-migrate` command-line entrypoint.

use std::process::ExitCode;

#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
mod cli;

#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
fn main() -> ExitCode {
  cli::run()
}

#[cfg(not(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
)))]
fn main() -> ExitCode {
  eprintln!(
    "chrono-rkyv-migrate: no legacy archive format selected; rebuild with exactly one of `legacy-16-le`, `legacy-16-be`, `legacy-32-le`, \
     `legacy-32-be`, `legacy-64-le`, or `legacy-64-be`"
  );
  ExitCode::FAILURE
}
