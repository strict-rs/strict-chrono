# tests — cross-checked integration tests and tzdata fixtures

Each Cargo integration-test target here is narrowly `#[cfg]`-gated so it only compiles where it can actually run, and two of them validate chrono against an external oracle rather than hardcoded expectations.
