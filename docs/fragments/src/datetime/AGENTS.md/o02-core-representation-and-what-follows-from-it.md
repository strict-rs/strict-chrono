## Core representation, and what follows from it

`DateTime<Tz>` holds exactly two fields: `datetime: NaiveDateTime` — **always in UTC** — and `offset: Tz::Offset`. No `Tz` value is stored, only its `Offset`, so the timezone object is reconstructed on demand via `TimeZone::from_offset`, and `Tz` never gates `Copy`/`Send`. The offset never changes the stored instant; it only derives the local wall-clock view, computed on demand as `datetime + offset.fix()`.

Two consequences drive most of the surprises here:

- **Equality, ordering, and hashing use the UTC instant only** — the offset is deliberately ignored. `PartialEq`/`PartialOrd` are cross-`Tz` (`DateTime<Tz>` vs `DateTime<Tz2>`) and compare `self.datetime` alone, so two values with different offsets but the same instant are `==` and hash-equal. Tests that also need to assert the offset compare it separately (a `norm()` helper).
- **The local view can fall just outside `NaiveDateTime`'s representable range** at the extremes, so there are two accessors. `naive_local()` **panics** when the local value is out of range; `overflowing_naive_local()` (`pub(crate)`) uses buffer space beyond the normal range and never panics. Nearly everything reads through the overflowing form — the `Datelike`/`Timelike` getters, `Debug`/`Display`, `format*`, `to_rfc2822`, and `to_rfc3339` — while `to_rfc3339_opts()` and the serde serializer use the panicking `naive_local()`. Its doc contract is explicit: the overflowing value must **never be exposed outside chrono**.
