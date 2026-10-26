use std::fs;
use std::process::Command;

use windows_bindgen::bindgen;

#[test]
fn gen_bindings() {
  let existing = fs::read_to_string(BINDINGS).unwrap();

  bindgen([
    "--out",
    BINDINGS,
    "--flat",
    "--sys",
    "--filter",
    "GetTimeZoneInformationForYear",
    "SystemTimeToFileTime",
    "SystemTimeToTzSpecificLocalTime",
    "TzSpecificLocalTimeToSystemTime",
  ]);

  // The generator invokes stable rustfmt, which ignores this repository's
  // nightly formatting options. Match the canonical `just fmt` toolchain before
  // comparing the generated artifact, so generation and formatting converge.
  let formatting = Command::new("rustup")
    .args(["run", "nightly", "rustfmt", BINDINGS])
    .status()
    .unwrap();
  assert!(formatting.success(), "formatting generated Windows bindings failed: {formatting}");

  // Check the output is the same as before.
  // Depending on the git configuration the file may have been checked out with `\r\n` newlines or
  // with `\n`. Compare line-by-line to ignore this difference.
  let mut new = fs::read_to_string(BINDINGS).unwrap();
  if existing.contains("\r\n") && !new.contains("\r\n") {
    new = new.replace("\n", "\r\n");
  } else if !existing.contains("\r\n") && new.contains("\r\n") {
    new = new.replace("\r\n", "\n");
  }

  similar_asserts::assert_eq!(existing, new);
  if !new.lines().eq(existing.lines()) {
    panic!("generated file `{BINDINGS}` is changed.");
  }
}

const BINDINGS: &str = "src/offset/local/win_bindings.rs";
