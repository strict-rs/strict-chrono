## Pipeline and the shared `Item` vocabulary

One enum, `Item<'a>` (in `mod.rs`), is the intermediate representation for both directions — which is exactly what lets a single format string round-trip:

format string → `StrftimeItems` (lexer, `strftime.rs`) → `Iterator<Item = Item>` → { render via `DelayedFormat` (`formatting.rs`) | parse via `parse` (`parse.rs`), filling a `Parsed` (`parsed.rs`) } → `Parsed::to_*` resolves to `NaiveDate` / `NaiveTime` / `NaiveDateTime` / `DateTime` / `FixedOffset`.

`Item` variants: `Literal` / `OwnedLiteral` (alloc) / `Space` / `OwnedSpace` (alloc) / `Numeric(Numeric, Pad)` / `Fixed(Fixed)` / `Error`. `Numeric` and `Fixed` are `#[non_exhaustive]` enums naming every field and format primitive (`Year`…`Timestamp`; `ShortMonthName`…`RFC3339`). Consumers build items through `StrftimeItems` or as `&[Item]` slices.
