//! Public migration API and downstream extension-seam tests.

use core::fmt;

use chrono_rkyv_migrate::DetectedSource;
use chrono_rkyv_migrate::LegacyDecode;
use chrono_rkyv_migrate::MigrationError;
use chrono_rkyv_migrate::SourceHint;
use chrono_rkyv_migrate::migrate;
use rancor::Fallible;
use rancor::Source;
use strict_test_support::TestFailure;
use strict_test_support::ensure;
use strict_test_support::ensure_contains;
use strict_test_support::ensure_ok;

/// Synthetic current archive used to deterministically exercise auto-detection.
#[derive(Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
struct Synthetic {
  /// Selects the downstream legacy decoder behavior.
  marker: u8,
  /// Logical test value.
  value:  i32,
}

impl LegacyDecode for Synthetic {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    if bytes == [0xa5] {
      return Ok(Self {
        marker: 3, value: 9
      });
    }
    let current = rkyv::from_bytes::<Self, rancor::Error>(bytes).map_err(|error| MigrationError::InvalidLegacyValue {
      detail: format!("synthetic legacy marker is invalid: {error}"),
    })?;
    match current.marker {
      0 => Ok(current),
      1 => Ok(Self {
        marker: current.marker,
        value:  8,
      }),
      _ => Err(MigrationError::InvalidLegacyValue {
        detail: "synthetic legacy marker is invalid".to_owned(),
      }),
    }
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

/// A logical value whose current-format serializer deliberately fails.
#[derive(Debug, PartialEq, rkyv::Archive, rkyv::Deserialize)]
struct EncodingFailure {
  /// Test marker retained only to give the archived type concrete data.
  marker: u8,
}

/// Error injected by [`EncodingFailure`]'s serializer.
#[derive(Debug)]
struct DeliberateEncodingFailure;

impl fmt::Display for DeliberateEncodingFailure {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("deliberate current encoding failure")
  }
}

impl std::error::Error for DeliberateEncodingFailure {}

impl<S> rkyv::Serialize<S> for EncodingFailure
where
  S: Fallible + ?Sized,
  S::Error: Source,
{
  fn serialize(&self, _serializer: &mut S) -> Result<Self::Resolver, S::Error> {
    Err(S::Error::new(DeliberateEncodingFailure))
  }
}

impl LegacyDecode for EncodingFailure {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    if bytes == [0x5a] {
      Ok(Self {
        marker: 7
      })
    } else {
      Err(MigrationError::InvalidLegacyValue {
        detail: "unexpected synthetic encoding-failure input".to_owned(),
      })
    }
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

/// Serializes one valid current synthetic archive.
fn current_bytes(marker: u8, value: i32) -> Result<rkyv::util::AlignedVec, TestFailure> {
  ensure_ok(
    rkyv::to_bytes::<rancor::Error>(&Synthetic {
      marker,
      value,
    }),
    "serialize current synthetic archive",
  )
}

#[test]
fn auto_detects_legacy_only_input() -> Result<(), TestFailure> {
  let migrated = ensure_ok(
    migrate::<Synthetic>(&[0xa5], SourceHint::Auto),
    "migrate legacy-only synthetic archive",
  )?;
  ensure(
    migrated.value
      == Synthetic {
        marker: 3, value: 9
      },
    "legacy-only input preserves the downstream decoder value",
  )?;
  ensure(
    migrated.detected_source == DetectedSource::Rkyv0_7,
    "legacy-only input reports rkyv 0.7",
  )
}

#[test]
fn auto_detects_current_only_input() -> Result<(), TestFailure> {
  let bytes = current_bytes(2, 7)?;
  let migrated = ensure_ok(
    migrate::<Synthetic>(&bytes, SourceHint::Auto),
    "migrate current-only synthetic archive",
  )?;
  ensure(
    migrated.value
      == Synthetic {
        marker: 2, value: 7
      },
    "current-only input preserves the current decoder value",
  )?;
  ensure(
    migrated.detected_source == DetectedSource::Rkyv0_8,
    "current-only input reports rkyv 0.8",
  )
}

#[test]
fn auto_reports_compatible_dual_valid_input() -> Result<(), TestFailure> {
  let bytes = current_bytes(0, 7)?;
  let migrated = ensure_ok(migrate::<Synthetic>(&bytes, SourceHint::Auto), "migrate dual-valid equal archive")?;
  ensure(
    migrated.value
      == Synthetic {
        marker: 0, value: 7
      },
    "compatible input preserves the shared logical value",
  )?;
  ensure(
    migrated.detected_source == DetectedSource::Compatible,
    "dual-valid equal input reports compatibility",
  )
}

#[test]
fn auto_rejects_different_dual_valid_values() -> Result<(), TestFailure> {
  let bytes = current_bytes(1, 7)?;
  ensure(
    matches!(migrate::<Synthetic>(&bytes, SourceHint::Auto), Err(MigrationError::AmbiguousFormat)),
    "dual-valid different input is never guessed",
  )
}

#[test]
fn auto_retains_both_unreadable_stages() -> Result<(), TestFailure> {
  let error = match migrate::<Synthetic>(&[], SourceHint::Auto) {
    Err(MigrationError::UnreadableInput {
      legacy,
      current,
    }) => (legacy, current),
    Ok(_) | Err(_) => {
      return Err(TestFailure::Condition {
        context: "empty input reports both unreadable decoder stages",
      });
    }
  };
  ensure_contains(&error.0, "synthetic legacy marker", "unreadable input retains the legacy failure")?;
  ensure(!error.1.is_empty(), "unreadable input retains the current failure")
}

#[test]
fn reports_current_format_encoding_failure() -> Result<(), TestFailure> {
  let error = migrate::<EncodingFailure>(&[0x5a], SourceHint::Rkyv0_7);
  match error {
    Err(MigrationError::CurrentEncoding {
      detail,
    }) => ensure_contains(
      &detail,
      "deliberate current encoding failure",
      "encoding failure retains the serializer diagnostic",
    ),
    Ok(_) | Err(_) => Err(TestFailure::Condition {
      context: "legacy success followed by serialization failure reports current encoding",
    }),
  }
}
