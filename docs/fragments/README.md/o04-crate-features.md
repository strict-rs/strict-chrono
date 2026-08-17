## Crate features

Default features:

* `alloc`: Enable features that depend on allocation (primarily string formatting).
* `std`: Enables functionality that depends on the standard library. This is a superset of `alloc`
  and adds interoperation with standard library types and traits.
* `clock`: Enables reading the local timezone (`Local`). This is a superset of `now`.
* `now`: Enables reading the system time (`now`).
* `wasmbind`: Interface with the JS Date API for the `wasm32` target.

Optional features:

* `serde`: Enable serialization/deserialization via [serde].
* `rkyv`: Enable serialization/deserialization via [rkyv]. Without an explicit width selector,
  rkyv uses its 32-bit archived-pointer fallback.
* `rkyv-16`: Enable `rkyv` with 16-bit archived pointers.
* `rkyv-32`: Enable `rkyv` with an explicit 32-bit archived-pointer selector.
* `rkyv-64`: Enable `rkyv` with 64-bit archived pointers.
* `rkyv-validation`: Enable `rkyv` and checked archive validation through `bytecheck`.
* `arbitrary`: Construct arbitrary instances of a type with the Arbitrary crate.
* `unstable-locales`: Enable localization. This adds various methods with a `_localized` suffix.
  The implementation and API may change or even be removed in a patch release. Feedback welcome.
* `oldtime`: This feature no longer has any effect; it used to offer compatibility with the `time` 0.1 crate.

The `rkyv-16`, `rkyv-32`, and `rkyv-64` selectors are mutually exclusive with one another. Each
selector implies the base `rkyv` integration, as does `rkyv-validation`.

`rkyv` `0.8` archives are not compatible with `rkyv` `0.7` archives. This workspace includes
[`chrono-rkyv-migrate`](./chrono-rkyv-migrate), an unpublished companion library and CLI that
checked-decodes the frozen `0.7` chrono layouts and emits canonical little-endian `0.8` archives.
Choose the legacy pointer width and endianness at compile time; application containers and custom
timezone schemas provide their own decoder through the companion's `LegacyDecode` extension trait.
See the [rkyv `0.8.0` release notes] for the upstream format boundary.

[serde]: https://github.com/serde-rs/serde
[rkyv]: https://github.com/rkyv/rkyv
[rkyv `0.8.0` release notes]: https://github.com/rkyv/rkyv/releases/tag/0.8.0
