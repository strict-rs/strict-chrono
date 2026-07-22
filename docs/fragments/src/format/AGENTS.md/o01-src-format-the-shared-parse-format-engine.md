# src/format — the shared parse + format engine

Every `to_string`/`format`/`parse*` path in the crate routes through here — `DateTime`, the `Naive*` types, and the offset types own no parsing or formatting logic of their own. Keep this module's public `Item`/`Numeric`/`Fixed`/`ParseError` surface stable; it is the shared contract those types depend on.
