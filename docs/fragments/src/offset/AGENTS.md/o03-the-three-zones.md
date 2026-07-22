## The three zones

- `Utc` (`utc.rs`) is a ZST; its offset is always zero and every mapping is `Single` (infallible). `Utc::now()` reads `SystemTime` (or `js_sys::Date` on wasm).
- `FixedOffset` (`fixed.rs`) stores a single `local_minus_utc: i32` limited to **±23:59:59** (`-86_400 < secs < 86_400`). Staying strictly within one day is a load-bearing invariant: it lets the `DateTime`/`Naive*` layers assume a local wall-clock is never more than a day from UTC. Mappings are always `Single`.
- `Local` (`local/mod.rs`) is a ZST that defers to a platform `inner` module chosen by `cfg`. All the real complexity is there.
