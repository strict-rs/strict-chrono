# `chrono-rkyv-migrate`

`chrono-rkyv-migrate` validates a caller-selected chrono root type in either a compiled `rkyv` `0.7` source format or current `rkyv` `0.8`, converts legacy values through checked chrono constructors, and writes canonical `rkyv` `0.8` little-endian bytes at the selected pointer width.

The crate is an isolated compatibility boundary: `chrono` itself depends only on `rkyv` `0.8`, while this unpublished companion owns the exact `rkyv` `0.7.46` dependency, frozen legacy representations, checked conversion logic, provenance-labelled fixtures, and CLI.

## Format selection

Exactly one source-mode feature selects the legacy pointer width and byte order:

- `legacy-16-le`
- `legacy-16-be`
- `legacy-32-le`
- `legacy-32-be`
- `legacy-64-le`
- `legacy-64-be`

Each mode enables the matching current chrono pointer width. Output is always canonical `rkyv` `0.8` little-endian data at that width. The default feature set is `["cli", "legacy-32-le"]`. A `--no-default-features` build exposes format and error vocabulary but deliberately omits migration operations because no legacy representation is selected.

Never combine source modes in one build. Raw archives contain no chrono root-type, archive-version, width, or byte-order marker, so the caller must supply the expected type and compile the matching width/endianness mode.

## Library API

`migrate::<T>(bytes, SourceHint)` validates both generations independently when `SourceHint::Auto` is selected:

1. If only one decoder succeeds, that source wins.
2. If both decoders produce the same logical value, the source is reported as `DetectedSource::Compatible`.
3. If both produce different values, migration returns `MigrationError::AmbiguousFormat`.
4. If neither succeeds, `MigrationError::UnreadableInput` retains both decoder stages.
5. Every success is re-encoded as current canonical bytes, including input that was already `rkyv` `0.8`.

Built-in decoders cover `TimeDelta`/`Duration`, `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `IsoWeek`, `Utc`, `FixedOffset`, `Local`, `Month`, `Weekday`, the built-in `DateTime<Tz>` variants, and explicitly deprecated `Date<Tz>` migration entrypoints. Scalar layouts are reconstructed through checked chrono constructors instead of deserializing frozen private state directly.

Application containers and custom timezone/archive schemas implement `LegacyDecode`. Its default current decoder uses checked `rkyv` `0.8` deserialization; applications with additional logical invariants can override that decoder as well.

## CLI

The maintained `bpaf` command surface migrates one direct built-in chrono root value:

```text
chrono-rkyv-migrate migrate \
  --type <supported-chrono-type> \
  --source-version <auto|0.7|0.8> \
  --input <path> \
  (--output <path> | --in-place)
```

Separate output uses create-new semantics. Pass `--replace` to overwrite an existing separate output explicitly. `--in-place` writes and syncs a sibling temporary file, preserves the input mode, and renames it over the input only after validation and current-format encoding succeed. The CLI prints the compiled source mode and canonical target format before writing.

Supported direct type tokens are `time-delta`, `duration`, `naive-date`, `naive-time`, `naive-datetime`, `iso-week`, `utc`, `fixed-offset`, `local`, `month`, `weekday`, `datetime-utc`, `datetime-fixed-offset`, `datetime-local`, `date-utc`, `date-fixed-offset`, and `date-local`.

The CLI intentionally does not recurse through directories or infer application schemas. Use the library extension seam for container types.

## Frozen fixtures

`fixtures/<legacy-mode>/manifest.toml` records the producer repository, exact pre-upgrade chrono revision, exact `rkyv` `0.7.46` release, toolchain, width, byte order, logical value, and producer kind for every checked-in binary. Ordinary fixtures were emitted by the pre-upgrade chrono derives. Deprecated `Date<Tz>` fixtures use the exact frozen private field layout because the old base `rkyv` feature itself selected 32-bit pointers and therefore could not expose its `Date<Tz>` derive in the 16-bit or 64-bit public feature modes.

Every mode test decodes the matching `0.7` fixtures, converts through checked constructors, checked-decodes the emitted `0.8` archive, and proves a second migration is byte-for-byte idempotent.
