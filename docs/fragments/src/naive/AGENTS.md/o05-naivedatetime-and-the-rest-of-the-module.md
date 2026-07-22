## `NaiveDateTime` and the rest of the module

`NaiveDateTime { date, time }` composes the two and delegates `Datelike`/`Timelike` through to them; its `checked_add/sub_offset` are the leap-second-preserving offset shifts, and its many `timestamp*` methods are deprecated in favour of `.and_utc()` on the resulting `DateTime<Utc>`. `mod.rs` also defines `NaiveWeek` (a `NaiveDate` plus a start `Weekday`, with `first_day`/`last_day`/`checked_*`/`days()` range) and the `Days(u64)` newtype.

serde: `naive::serde` re-exports `ts_seconds`/`ts_milliseconds`/`ts_microseconds`/`ts_nanoseconds` (and `_option` variants) for `NaiveDateTime`. These are **distinct** from the identically named adapters under `crate::serde`, which target `DateTime<Utc>`; don't conflate them.
