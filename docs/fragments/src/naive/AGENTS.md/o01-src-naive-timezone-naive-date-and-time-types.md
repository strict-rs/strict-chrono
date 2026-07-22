# src/naive — timezone-naive date and time types

Owns the timezone-agnostic building blocks the rest of the crate is composed from: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `IsoWeek`, and the supporting `NaiveWeek` / `Days` newtypes. `mod.rs` re-exports them and declares the `serde` submodule; the type impls live in `date/`, `time/`, `datetime/`, `isoweek.rs`, and the bit-packing machinery in `internals.rs`.
