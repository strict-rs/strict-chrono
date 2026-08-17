## Generated file — do not hand-edit

`local/win_bindings.rs` is pure `windows-bindgen` output containing Win32 FFI shims via `windows_link::link!`; the handwritten `local/mod.rs` owns the narrow lint boundary for that generated module. Regenerate the bindings through `tests/win_bindings.rs` rather than editing them by hand; the integration test owns the API filter list and fails if the checked-in output drifts from the generator.
