//! Checked migration from legacy chrono `rkyv` `0.7` archives to canonical
//! `rkyv` `0.8` archives.

#![forbid(unsafe_code)]

#[cfg(any(
  all(feature = "legacy-16-le", feature = "legacy-16-be"),
  all(feature = "legacy-16-le", feature = "legacy-32-le"),
  all(feature = "legacy-16-le", feature = "legacy-32-be"),
  all(feature = "legacy-16-le", feature = "legacy-64-le"),
  all(feature = "legacy-16-le", feature = "legacy-64-be"),
  all(feature = "legacy-16-be", feature = "legacy-32-le"),
  all(feature = "legacy-16-be", feature = "legacy-32-be"),
  all(feature = "legacy-16-be", feature = "legacy-64-le"),
  all(feature = "legacy-16-be", feature = "legacy-64-be"),
  all(feature = "legacy-32-le", feature = "legacy-32-be"),
  all(feature = "legacy-32-le", feature = "legacy-64-le"),
  all(feature = "legacy-32-le", feature = "legacy-64-be"),
  all(feature = "legacy-32-be", feature = "legacy-64-le"),
  all(feature = "legacy-32-be", feature = "legacy-64-be"),
  all(feature = "legacy-64-le", feature = "legacy-64-be"),
))]
compile_error!("exactly one `legacy-{16,32,64}-{le,be}` feature may be enabled");

#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
mod legacy;

/// Archived pointer-width selector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerWidth {
  /// 16-bit archived pointer representation.
  Bits16,
  /// 32-bit archived pointer representation.
  Bits32,
  /// 64-bit archived pointer representation.
  Bits64,
}

/// Archived byte order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endianness {
  /// Little-endian representation.
  Little,
  /// Big-endian representation.
  Big,
}

/// Archive format generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveVersion {
  /// Legacy `rkyv` `0.7`.
  Rkyv0_7,
  /// Current `rkyv` `0.8`.
  Rkyv0_8,
}

/// Caller-supplied source-version selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceHint {
  /// Validate both supported archive generations independently.
  Auto,
  /// Decode only legacy `rkyv` `0.7`.
  Rkyv0_7,
  /// Decode only current `rkyv` `0.8`.
  Rkyv0_8,
}

/// Archive generation detected during migration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetectedSource {
  /// Only the legacy decoder accepted the input.
  Rkyv0_7,
  /// Only the current decoder accepted the input.
  Rkyv0_8,
  /// Both decoders produced the same logical value.
  Compatible,
}

/// Complete archive representation facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArchiveFormat {
  /// Archive generation.
  pub version:    ArchiveVersion,
  /// Archived pointer width.
  pub width:      PointerWidth,
  /// Archived byte order.
  pub endianness: Endianness,
}

/// Successful logical migration and canonical output bytes.
#[derive(Clone, Debug)]
pub struct MigratedArchive<T> {
  /// Decoded logical chrono value.
  pub value:           T,
  /// Canonical aligned `rkyv` `0.8` bytes.
  pub canonical_bytes: rkyv::util::AlignedVec,
  /// Detected source generation.
  pub detected_source: DetectedSource,
  /// Canonical target representation.
  pub target_format:   ArchiveFormat,
}

/// Typed archive migration failure.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MigrationError {
  /// Neither supported source decoder accepted the input.
  #[error("archive input is unreadable as rkyv 0.7 ({legacy}) or rkyv 0.8 ({current})")]
  UnreadableInput {
    /// Legacy decoding stage.
    legacy:  String,
    /// Current decoding stage.
    current: String,
  },
  /// Both decoders accepted the input but produced different logical values.
  #[error("archive input is valid as both rkyv 0.7 and rkyv 0.8 with different logical values")]
  AmbiguousFormat,
  /// A frozen legacy representation violates chrono's public invariants.
  #[error("legacy archive value is invalid: {detail}")]
  InvalidLegacyValue {
    /// Rejected invariant.
    detail: String,
  },
  /// A current archive is structurally invalid or violates logical invariants.
  #[error("current archive value is invalid: {detail}")]
  InvalidCurrentValue {
    /// Rejected structure or invariant.
    detail: String,
  },
  /// Current-format canonical encoding failed.
  #[error("canonical rkyv 0.8 encoding failed: {detail}")]
  CurrentEncoding {
    /// Encoding failure.
    detail: String,
  },
  /// The direct CLI does not expose this root type.
  #[error("unsupported direct CLI chrono type `{type_name}`")]
  UnsupportedDirectType {
    /// Rejected CLI type token.
    type_name: String,
  },
}

/// Downstream extension seam for decoding application-owned `rkyv` `0.7`
/// schemas into one current logical value.
pub trait LegacyDecode: Sized {
  /// Decode and validate one legacy archive.
  ///
  /// # Errors
  ///
  /// Returns a typed migration error when bytes or logical invariants are
  /// invalid.
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError>;

  /// Checked-decode one current `rkyv` `0.8` archive.
  ///
  /// The default uses `rkyv`'s checked high-level decoder. Implementations for
  /// chrono's built-in types additionally reconstruct private scalar layouts
  /// through public checked constructors. Application types with invariants
  /// beyond their `CheckBytes` implementation can override this method.
  ///
  /// # Errors
  ///
  /// Returns [`MigrationError::InvalidCurrentValue`] when structural checking,
  /// deserialization, or application-level invariant validation fails.
  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError>
  where
    Self: rkyv::Archive,
    Self::Archived: for<'bytes> rkyv::bytecheck::CheckBytes<
        rkyv::api::high::HighValidator<'bytes, rancor::Error>,
      > + rkyv::Deserialize<
        Self,
        rkyv::rancor::Strategy<rkyv::de::Pool, rancor::Error>,
      >,
  {
    rkyv::from_bytes::<Self, rancor::Error>(bytes).map_err(|error| {
      MigrationError::InvalidCurrentValue {
        detail: error.to_string(),
      }
    })
  }

  /// Returns whether two independently decoded values represent the same
  /// logical value.
  ///
  /// This hook lets application schemas define semantic equality for types
  /// such as `Local` that intentionally do not implement `PartialEq`.
  fn logically_equals(&self, other: &Self) -> bool;
}

/// Validates one supported source archive and re-encodes its logical value as
/// canonical `rkyv` `0.8` bytes.
///
/// `SourceHint::Auto` runs the current and legacy decoders independently. It
/// never chooses between two different logical values.
///
/// # Errors
///
/// Returns a typed error when neither decoder accepts the bytes, both accept
/// them with different logical values, a legacy representation violates
/// chrono's invariants, or canonical encoding fails.
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
pub fn migrate<T>(
  bytes: &[u8],
  source_hint: SourceHint,
) -> Result<MigratedArchive<T>, MigrationError>
where
  T: LegacyDecode
    + rkyv::Archive
    + for<'arena> rkyv::Serialize<
      rkyv::api::high::HighSerializer<
        rkyv::util::AlignedVec,
        rkyv::ser::allocator::ArenaHandle<'arena>,
        rancor::Error,
      >,
    >,
  T::Archived: for<'bytes> rkyv::bytecheck::CheckBytes<
      rkyv::api::high::HighValidator<'bytes, rancor::Error>,
    > + rkyv::Deserialize<
      T,
      rkyv::rancor::Strategy<rkyv::de::Pool, rancor::Error>,
    >,
{
  let (value, detected_source) = match source_hint {
    SourceHint::Rkyv0_7 => (
      T::decode_legacy(bytes)?,
      DetectedSource::Rkyv0_7,
    ),
    SourceHint::Rkyv0_8 => (
      T::decode_current(bytes).map_err(|current| MigrationError::UnreadableInput {
        legacy: "legacy decoder was not run".to_owned(),
        current: current.to_string(),
      })?,
      DetectedSource::Rkyv0_8,
    ),
    SourceHint::Auto => {
      let current = T::decode_current(bytes);
      let legacy = T::decode_legacy(bytes);
      match (legacy, current) {
        (Ok(legacy), Ok(current)) if legacy.logically_equals(&current) => {
          (legacy, DetectedSource::Compatible)
        }
        (Ok(_), Ok(_)) => return Err(MigrationError::AmbiguousFormat),
        (Ok(legacy), Err(_)) => (legacy, DetectedSource::Rkyv0_7),
        (Err(_), Ok(current)) => (current, DetectedSource::Rkyv0_8),
        (Err(legacy), Err(current)) => {
          return Err(MigrationError::UnreadableInput {
            legacy: legacy.to_string(),
            current: current.to_string(),
          });
        }
      }
    }
  };

  let canonical_bytes =
    rkyv::to_bytes::<rancor::Error>(&value).map_err(|error| MigrationError::CurrentEncoding {
      detail: error.to_string(),
    })?;
  Ok(MigratedArchive {
    value,
    canonical_bytes,
    detected_source,
    target_format: target_format(),
  })
}

/// Migrates the deprecated `Date<Utc>` root representation.
///
/// # Errors
///
/// Returns the same typed migration errors as [`migrate`].
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
#[allow(
  deprecated,
  reason = "this explicitly deprecated entrypoint exists only to migrate historical Date archives"
)]
#[deprecated(note = "migrate DateTime<Utc> or NaiveDate archives for new data")]
pub fn migrate_date_utc(
  bytes: &[u8],
  source_hint: SourceHint,
) -> Result<MigratedArchive<chrono::Date<chrono::Utc>>, MigrationError> {
  migrate(bytes, source_hint)
}

/// Migrates the deprecated `Date<FixedOffset>` root representation.
///
/// # Errors
///
/// Returns the same typed migration errors as [`migrate`].
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
#[allow(
  deprecated,
  reason = "this explicitly deprecated entrypoint exists only to migrate historical Date archives"
)]
#[deprecated(note = "migrate DateTime<FixedOffset> or NaiveDate archives for new data")]
pub fn migrate_date_fixed(
  bytes: &[u8],
  source_hint: SourceHint,
) -> Result<MigratedArchive<chrono::Date<chrono::FixedOffset>>, MigrationError> {
  migrate(bytes, source_hint)
}

/// Migrates the deprecated `Date<Local>` root representation.
///
/// # Errors
///
/// Returns the same typed migration errors as [`migrate`].
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
#[allow(
  deprecated,
  reason = "this explicitly deprecated entrypoint exists only to migrate historical Date archives"
)]
#[deprecated(note = "migrate DateTime<Local> or NaiveDate archives for new data")]
pub fn migrate_date_local(
  bytes: &[u8],
  source_hint: SourceHint,
) -> Result<MigratedArchive<chrono::Date<chrono::Local>>, MigrationError> {
  migrate(bytes, source_hint)
}

/// Selected source representation.
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
#[must_use]
pub const fn source_format() -> ArchiveFormat {
  ArchiveFormat {
    version: ArchiveVersion::Rkyv0_7,
    width: selected_width(),
    endianness: selected_endianness(),
  }
}

/// Canonical target representation.
#[cfg(any(
  feature = "legacy-16-le",
  feature = "legacy-16-be",
  feature = "legacy-32-le",
  feature = "legacy-32-be",
  feature = "legacy-64-le",
  feature = "legacy-64-be",
))]
#[must_use]
pub const fn target_format() -> ArchiveFormat {
  ArchiveFormat {
    version: ArchiveVersion::Rkyv0_8,
    width: selected_width(),
    endianness: Endianness::Little,
  }
}

#[cfg(any(feature = "legacy-16-le", feature = "legacy-16-be"))]
const fn selected_width() -> PointerWidth {
  PointerWidth::Bits16
}

#[cfg(any(feature = "legacy-32-le", feature = "legacy-32-be"))]
const fn selected_width() -> PointerWidth {
  PointerWidth::Bits32
}

#[cfg(any(feature = "legacy-64-le", feature = "legacy-64-be"))]
const fn selected_width() -> PointerWidth {
  PointerWidth::Bits64
}

#[cfg(any(feature = "legacy-16-le", feature = "legacy-32-le", feature = "legacy-64-le"))]
const fn selected_endianness() -> Endianness {
  Endianness::Little
}

#[cfg(any(feature = "legacy-16-be", feature = "legacy-32-be", feature = "legacy-64-be"))]
const fn selected_endianness() -> Endianness {
  Endianness::Big
}
