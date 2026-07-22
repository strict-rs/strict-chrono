## Panics vs `Option`

Fallible operations split cleanly: the `checked_*` methods (`checked_add/sub_signed`, `checked_add/sub_months`, `checked_add/sub_days`) and `years_since` return `Option`, while `signed_duration_since` returns a `TimeDelta` and never overflows; `naive_local()` and the arithmetic operators (`Add`/`Sub`/`AddAssign`/`SubAssign`) panic. Every panicking operator is `#[track_caller]` and delegates to its `checked_*` sibling with `.expect(...)`. `Add`/`Sub` of a `FixedOffset` shift the stored UTC `datetime` and leave the `offset` field unchanged.

`with_*` setters (from `Datelike`/`Timelike`) route through the private `map_local` helper: it applies the change to `overflowing_naive_local()`, re-resolves through `timezone().from_local_datetime(..).single()`, then filters to `MIN_UTC..=MAX_UTC`. That is why a setter returns `None` for a nonexistent or ambiguous local time (a DST transition) or an out-of-range result.

Subtlety to preserve: `checked_add_months`/`checked_add_days` (and the `sub` forms) intentionally exploit the `Months(0)`/`Days(0)` fast paths in `NaiveDate`, which skip validation, so a zero-length add can return `Some` even when the local datetime is out of range. `checked_add_days` additionally short-circuits `Days::new(0) => Some(self)`.
