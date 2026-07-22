## Cross-cutting patterns worth knowing

- **Const-context plumbing.** `try_opt!` / `expect()` appear everywhere because most constructors are `const fn` and cannot use `?` / `.expect()`.
- **Leap seconds** are represented uniformly as a nanosecond field ≥ 1e9 (only when `secs % 60 == 59`); the details live in `naive/` and `format/`.
- **The out-of-range buffer.** `NaiveDate` reserves one year of headroom (`BEFORE_MIN`/`AFTER_MAX`) so a local time just outside the representable range can exist transiently during offset math without panicking; `DateTime::overflowing_naive_local` and `NaiveDateTime::overflowing_*_offset` rely on it. Never expose such a value.
- **Feature gating.** `alloc` unlocks string formatting, `std` adds OS/`SystemTime` interop, `clock` enables `Local`, and the `rkyv-16`/`rkyv-32`/`rkyv-64` features are mutually exclusive. Optional integrations (`serde`, `rkyv`, `arbitrary`, `defmt`) are gated at the crate root and mirrored per type.
