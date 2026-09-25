//! CLI behavior when no legacy archive representation is compiled.

#![cfg(all(
  feature = "cli",
  not(any(
    feature = "legacy-16-le",
    feature = "legacy-16-be",
    feature = "legacy-32-le",
    feature = "legacy-32-be",
    feature = "legacy-64-le",
    feature = "legacy-64-be",
  ))
))]

use std::fs;
use std::process::Command;

use strict_test_support::ConditionFailure;
use strict_test_support::TempDir;
use strict_test_support::TestFailure;
use strict_test_support::ensure;

/// Assertion, fixture, and I/O failures from the CLI regression test.
#[derive(Debug, thiserror::Error)]
enum CliTestFailure {
  /// The observed command behavior did not match its contract.
  #[error(transparent)]
  Condition(#[from] ConditionFailure),
  /// Temporary fixture creation failed.
  #[error(transparent)]
  Fixture(#[from] TestFailure),
  /// A process or filesystem operation failed.
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

#[test]
fn missing_legacy_format_rejects_migration_without_changing_files() -> Result<(), CliTestFailure> {
  let directory = TempDir::new("chrono-rkyv-migrate-no-format")?;
  let input = directory.path().join("input.bin");
  let output = directory.path().join("output.bin");
  fs::write(&input, b"original archive")?;

  for destination in [vec!["--output", "output.bin"], vec!["--in-place"]] {
    let result = Command::new(env!("CARGO_BIN_EXE_chrono-rkyv-migrate"))
      .current_dir(directory.path())
      .args(["migrate", "--type", "month", "--source-version", "0.8", "--input", "input.bin"])
      .args(destination)
      .output()?;

    ensure(result.status.code() == Some(1), "unconfigured CLI reports failure")?;
    ensure(result.stdout.is_empty(), "unconfigured CLI emits no payload output")?;
    let diagnostic = String::from_utf8_lossy(&result.stderr);
    ensure(
      diagnostic.contains("no legacy archive format selected"),
      "diagnostic explains the missing build configuration",
    )?;
    for feature in [
      "legacy-16-le", "legacy-16-be", "legacy-32-le", "legacy-32-be", "legacy-64-le", "legacy-64-be",
    ] {
      ensure(diagnostic.contains(feature), "diagnostic names every supported format selector")?;
    }
    ensure(fs::read(&input)? == b"original archive", "unconfigured CLI preserves the input")?;
    ensure(!output.try_exists()?, "unconfigured CLI creates no output archive")?;
    ensure(
      fs::read_dir(directory.path())?.count() == 1,
      "unconfigured CLI creates no temporary files",
    )?;
  }
  Ok(())
}
