# src/ — crate root and shared primitives

`lib.rs` is the crate root: it owns feature gating (`no_std` when neither `std` nor `test` is active), the `prelude`, the public re-export surface, and two small but pervasive helpers — the `try_opt!` macro and the `pub(crate) const fn expect()`, which stand in for `?` and `.expect()` in `const` context (that is why so much of the crate threads `Option` through `try_opt!` by hand).
