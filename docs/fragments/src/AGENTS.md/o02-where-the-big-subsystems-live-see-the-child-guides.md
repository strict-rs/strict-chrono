## Where the big subsystems live — see the child guides

The four subsystems each have their own `AGENTS.md`; consult those rather than duplicating their internals here:

- `datetime/` — `DateTime<Tz>`, the timezone-aware instant. See `datetime/AGENTS.md`.
- `format/` — the shared `strftime` parse/format engine (`Item`, `StrftimeItems`, `Parsed`). See `format/AGENTS.md`.
- `naive/` — the timezone-naive types and their bit-packed representation. See `naive/AGENTS.md`.
- `offset/` — the `TimeZone`/`Offset` traits, `MappedLocalTime`, and `Utc`/`FixedOffset`/`Local`. See `offset/AGENTS.md`.
