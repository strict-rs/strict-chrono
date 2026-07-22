## cfg gating and consumers

`alloc` gates `DelayedFormat`, the `Owned*` item variants, the deprecated free `format`/`format_item`, and `write_rfc2822`/`write_rfc3339` (`write_rfc3339` is also compiled under `serde`). `unstable-locales` gates `Locale`, `new_with_locale`, and the `format_localized*` paths. The `Error` impl on `ParseError` is under `core-error`/`std`.

Consumers import from here rather than reimplementing: `datetime` and the `naive/*` types build `StrftimeItems` or item slices and hand them to `DelayedFormat` (format) or `parse`/`Parsed` (parse), relying on `Parsed::to_*`; `offset` uses `parse`/`Parsed`/`StrftimeItems`, and `offset/fixed.rs` uses `scan` + `OUT_OF_RANGE` directly.
