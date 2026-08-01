//! Frozen pre-upgrade archive compatibility tests.

#![allow(
  deprecated,
  reason = "the fixture suite verifies the explicitly deprecated Date migration entrypoints"
)]

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{
  Date, DateTime, Datelike, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime,
  TimeDelta, Utc, Weekday,
};
use chrono_rkyv_migrate::{
  migrate, migrate_date_fixed, migrate_date_local, migrate_date_utc, source_format,
  target_format, DetectedSource, Endianness, LegacyDecode, PointerWidth, SourceHint,
};
use strict_test_support::{
  ensure, ensure_contains, ensure_eq, ensure_ok, ensure_some, TestFailure,
};

/// Exact source revision used to produce the checked-in legacy bytes.
const PRODUCER_REVISION: &str = "cc3ba6e0a512101df43b31a8f5bd230a7695c01b";

/// Returns the fixture directory selected by the active legacy mode.
fn mode_directory() -> &'static str {
  match (source_format().width, source_format().endianness) {
    (PointerWidth::Bits16, Endianness::Little) => "legacy-16-le",
    (PointerWidth::Bits16, Endianness::Big) => "legacy-16-be",
    (PointerWidth::Bits32, Endianness::Little) => "legacy-32-le",
    (PointerWidth::Bits32, Endianness::Big) => "legacy-32-be",
    (PointerWidth::Bits64, Endianness::Little) => "legacy-64-le",
    (PointerWidth::Bits64, Endianness::Big) => "legacy-64-be",
  }
}

/// Returns the active mode's fixture root.
fn fixture_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join(mode_directory())
}

/// Loads one provenance-labelled fixture.
fn fixture(name: &str) -> Result<Vec<u8>, TestFailure> {
  Ok(fs::read(fixture_root().join(name))?)
}

#[test]
fn fixture_manifest_records_exact_provenance() -> Result<(), TestFailure> {
  let manifest = fs::read_to_string(fixture_root().join("manifest.toml"))?;
  ensure_contains(
    &manifest,
    &format!("producer-revision = \"{PRODUCER_REVISION}\""),
    "manifest records the exact pre-upgrade source revision",
  )?;
  ensure_contains(
    &manifest,
    "rkyv-version = \"0.7.46\"",
    "manifest records the exact legacy rkyv release",
  )?;
  ensure_contains(
    &manifest,
    &format!("pointer-width = {:?}", match source_format().width {
      PointerWidth::Bits16 => 16,
      PointerWidth::Bits32 => 32,
      PointerWidth::Bits64 => 64,
    }),
    "manifest records the selected pointer width",
  )?;
  let expected_endianness = match source_format().endianness {
    Endianness::Little => "endianness = \"little\"",
    Endianness::Big => "endianness = \"big\"",
  };
  ensure_contains(
    &manifest,
    expected_endianness,
    "manifest records the selected legacy endianness",
  )?;
  ensure_eq(
    &manifest.matches("[[fixtures]]").count(),
    &48_usize,
    "manifest lists every direct-type fixture",
  )?;
  ensure_eq(
    &manifest
      .matches("producer-kind = \"frozen-pre-upgrade-date-layout\"")
      .count(),
    &9_usize,
    "manifest identifies every frozen deprecated Date fixture",
  )
}

#[test]
fn migrates_every_frozen_legacy_fixture_idempotently() -> Result<(), TestFailure> {
  macro_rules! verify {
    ($file:literal, $target:ty, $expected:expr) => {{
      let bytes = fixture($file)?;
      let expected: $target = $expected;
      let migrated = ensure_ok(
        migrate::<$target>(&bytes, SourceHint::Rkyv0_7),
        "legacy fixture migrates",
      )?;
      ensure(
        migrated.value.logically_equals(&expected),
        "legacy fixture preserves its logical chrono value",
      )?;
      ensure(
        migrated.detected_source == DetectedSource::Rkyv0_7,
        "forced legacy migration reports rkyv 0.7",
      )?;
      ensure(
        migrated.target_format == target_format(),
        "migration reports the compiled canonical target format",
      )?;
      let repeated = ensure_ok(
        migrate::<$target>(&migrated.canonical_bytes, SourceHint::Rkyv0_8),
        "canonical archive migrates again",
      )?;
      ensure(
        repeated.value.logically_equals(&expected),
        "repeated migration preserves the logical chrono value",
      )?;
      ensure(
        repeated.canonical_bytes.as_slice() == migrated.canonical_bytes.as_slice(),
        "repeated migration is byte-for-byte idempotent",
      )?;
    }};
  }

  macro_rules! verify_date {
    ($file:literal, $migrate:path, $expected:expr) => {{
      let bytes = fixture($file)?;
      let expected = $expected;
      let migrated = ensure_ok(
        $migrate(&bytes, SourceHint::Rkyv0_7),
        "deprecated Date fixture migrates",
      )?;
      ensure(
        migrated.value.logically_equals(&expected),
        "deprecated Date fixture preserves its logical chrono value",
      )?;
      let repeated = ensure_ok(
        $migrate(&migrated.canonical_bytes, SourceHint::Rkyv0_8),
        "canonical deprecated Date archive migrates again",
      )?;
      ensure(
        repeated.value.logically_equals(&expected),
        "repeated deprecated Date migration preserves its logical value",
      )?;
      ensure(
        repeated.canonical_bytes.as_slice() == migrated.canonical_bytes.as_slice(),
        "deprecated Date migration is byte-for-byte idempotent",
      )?;
    }};
  }

  let representative_date = ensure_some(
    NaiveDate::from_ymd_opt(2024, 2, 29),
    "construct representative date",
  )?;
  let representative_time = ensure_some(
    NaiveTime::from_hms_nano_opt(12, 34, 56, 789_000_000),
    "construct representative time",
  )?;
  let leap_time = ensure_some(
    NaiveTime::from_hms_nano_opt(23, 59, 59, 1_500_000_000),
    "construct leap-second time",
  )?;
  let maximum_time = ensure_some(
    NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999),
    "construct maximum time",
  )?;
  let representative_naive = NaiveDateTime::new(representative_date, representative_time);
  let leap_naive = NaiveDateTime::new(representative_date, leap_time);
  let minimum_offset = ensure_some(
    FixedOffset::west_opt(86_399),
    "construct minimum fixed offset",
  )?;
  let zero_offset = ensure_some(FixedOffset::east_opt(0), "construct zero fixed offset")?;
  let maximum_offset = ensure_some(
    FixedOffset::east_opt(86_399),
    "construct maximum fixed offset",
  )?;
  let representative_offset = ensure_some(
    FixedOffset::east_opt(19_800),
    "construct representative fixed offset",
  )?;
  let leap_offset = ensure_some(
    FixedOffset::west_opt(18_000),
    "construct leap-second fixed offset",
  )?;

  verify!("time-delta-min.bin", TimeDelta, TimeDelta::MIN);
  verify!(
    "time-delta-negative.bin",
    TimeDelta,
    ensure_some(
      TimeDelta::new(-2, 750_000_000),
      "construct negative duration",
    )?
  );
  verify!("time-delta-zero.bin", TimeDelta, TimeDelta::zero());
  verify!("time-delta-max.bin", TimeDelta, TimeDelta::MAX);

  verify!("naive-date-min.bin", NaiveDate, NaiveDate::MIN);
  verify!(
    "naive-date-representative.bin",
    NaiveDate,
    representative_date
  );
  verify!("naive-date-max.bin", NaiveDate, NaiveDate::MAX);

  verify!("naive-time-min.bin", NaiveTime, NaiveTime::MIN);
  verify!(
    "naive-time-representative.bin",
    NaiveTime,
    representative_time
  );
  verify!("naive-time-leap.bin", NaiveTime, leap_time);
  verify!("naive-time-max.bin", NaiveTime, maximum_time);

  verify!(
    "naive-datetime-min.bin",
    NaiveDateTime,
    NaiveDateTime::MIN
  );
  verify!(
    "naive-datetime-representative.bin",
    NaiveDateTime,
    representative_naive
  );
  verify!("naive-datetime-leap.bin", NaiveDateTime, leap_naive);
  verify!(
    "naive-datetime-max.bin",
    NaiveDateTime,
    NaiveDateTime::MAX
  );

  verify!("iso-week-min.bin", chrono::IsoWeek, NaiveDate::MIN.iso_week());
  verify!(
    "iso-week-representative.bin",
    chrono::IsoWeek,
    representative_date.iso_week()
  );
  verify!("iso-week-max.bin", chrono::IsoWeek, NaiveDate::MAX.iso_week());

  verify!("utc.bin", Utc, Utc);
  verify!("local.bin", Local, Local);
  verify!("fixed-offset-min.bin", FixedOffset, minimum_offset);
  verify!("fixed-offset-zero.bin", FixedOffset, zero_offset);
  verify!("fixed-offset-max.bin", FixedOffset, maximum_offset);

  verify!("month-min.bin", Month, Month::January);
  verify!("month-max.bin", Month, Month::December);
  verify!("weekday-min.bin", Weekday, Weekday::Mon);
  verify!("weekday-max.bin", Weekday, Weekday::Sun);

  verify!("datetime-utc-min.bin", DateTime<Utc>, DateTime::<Utc>::MIN_UTC);
  verify!(
    "datetime-utc-representative.bin",
    DateTime<Utc>,
    DateTime::from_naive_utc_and_offset(representative_naive, Utc)
  );
  verify!(
    "datetime-utc-leap.bin",
    DateTime<Utc>,
    DateTime::from_naive_utc_and_offset(leap_naive, Utc)
  );
  verify!("datetime-utc-max.bin", DateTime<Utc>, DateTime::<Utc>::MAX_UTC);

  verify!(
    "datetime-fixed-min.bin",
    DateTime<FixedOffset>,
    DateTime::from_naive_utc_and_offset(NaiveDateTime::MIN, zero_offset)
  );
  verify!(
    "datetime-fixed-representative.bin",
    DateTime<FixedOffset>,
    DateTime::from_naive_utc_and_offset(representative_naive, representative_offset)
  );
  verify!(
    "datetime-fixed-leap.bin",
    DateTime<FixedOffset>,
    DateTime::from_naive_utc_and_offset(leap_naive, leap_offset)
  );
  verify!(
    "datetime-fixed-max.bin",
    DateTime<FixedOffset>,
    DateTime::from_naive_utc_and_offset(NaiveDateTime::MAX, zero_offset)
  );

  verify!(
    "datetime-local-min.bin",
    DateTime<Local>,
    DateTime::from_naive_utc_and_offset(NaiveDateTime::MIN, zero_offset)
  );
  verify!(
    "datetime-local-representative.bin",
    DateTime<Local>,
    DateTime::from_naive_utc_and_offset(representative_naive, representative_offset)
  );
  verify!(
    "datetime-local-leap.bin",
    DateTime<Local>,
    DateTime::from_naive_utc_and_offset(leap_naive, leap_offset)
  );
  verify!(
    "datetime-local-max.bin",
    DateTime<Local>,
    DateTime::from_naive_utc_and_offset(NaiveDateTime::MAX, zero_offset)
  );

  verify_date!("date-utc-min.bin", migrate_date_utc, Date::<Utc>::MIN_UTC);
  verify_date!(
    "date-utc-representative.bin",
    migrate_date_utc,
    Date::<Utc>::from_utc(representative_date, Utc)
  );
  verify_date!("date-utc-max.bin", migrate_date_utc, Date::<Utc>::MAX_UTC);

  verify_date!(
    "date-fixed-min.bin",
    migrate_date_fixed,
    Date::<FixedOffset>::from_utc(NaiveDate::MIN, zero_offset)
  );
  verify_date!(
    "date-fixed-representative.bin",
    migrate_date_fixed,
    Date::<FixedOffset>::from_utc(representative_date, representative_offset)
  );
  verify_date!(
    "date-fixed-max.bin",
    migrate_date_fixed,
    Date::<FixedOffset>::from_utc(NaiveDate::MAX, zero_offset)
  );

  verify_date!(
    "date-local-min.bin",
    migrate_date_local,
    Date::<Local>::from_utc(NaiveDate::MIN, zero_offset)
  );
  verify_date!(
    "date-local-representative.bin",
    migrate_date_local,
    Date::<Local>::from_utc(representative_date, representative_offset)
  );
  verify_date!(
    "date-local-max.bin",
    migrate_date_local,
    Date::<Local>::from_utc(NaiveDate::MAX, zero_offset)
  );

  Ok(())
}
