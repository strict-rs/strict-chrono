//! Native platform clock conversion tests.

use std::error::Error;
use std::time::Duration;
#[cfg(feature = "now")]
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::DateTime;
use crate::DateTimeConversionError;
use crate::TimeDelta;
use crate::Utc;

#[test]
fn preserves_signed_seconds_and_nanoseconds() {
  let cases = [
    (Duration::ZERO, 0, 0, 0, 0),
    // Windows SystemTime stores 100 ns intervals, so it cannot supply 1 ns samples.
    #[cfg(not(windows))]
    (Duration::from_nanos(1), 0, 1, -1, 999_999_999),
    (Duration::from_nanos(100), 0, 100, -1, 999_999_900),
    (Duration::from_secs(1), 1, 0, -1, 0),
    #[cfg(not(windows))]
    (Duration::new(1, 1), 1, 1, -2, 999_999_999),
    (Duration::new(1, 100), 1, 100, -2, 999_999_900),
    (
      Duration::new(999_999_999, 999_999_000),
      999_999_999,
      999_999_000,
      -1_000_000_000,
      1_000,
    ),
  ];
  for (distance, positive_seconds, positive_nanos, negative_seconds, negative_nanos) in cases {
    let after = UNIX_EPOCH.checked_add(distance).unwrap();
    let positive = DateTime::<Utc>::from_system_time(after).unwrap();
    assert_eq!(Some(positive), DateTime::from_timestamp(positive_seconds, positive_nanos));
    assert_eq!(positive, DateTime::<Utc>::from(after));

    // Some platforms cannot represent dates before the Unix epoch.
    if let Some(before) = UNIX_EPOCH.checked_sub(distance) {
      let negative = DateTime::<Utc>::from_system_time(before).unwrap();
      assert_eq!(Some(negative), DateTime::from_timestamp(negative_seconds, negative_nanos));
      assert_eq!(negative, DateTime::<Utc>::from(before));
    }
  }
}

#[test]
fn preserves_calendar_boundaries_and_rejects_adjacent_samples() {
  let maximum = DateTime::<Utc>::MAX_UTC
    .signed_duration_since(DateTime::UNIX_EPOCH)
    .to_std()
    .unwrap();
  if let Some(last) = UNIX_EPOCH.checked_add(maximum) {
    assert_eq!(DateTime::<Utc>::from_system_time(last).unwrap(), DateTime::<Utc>::MAX_UTC);
    if let Some(sample) = last.checked_add(Duration::from_nanos(1)) {
      let failure = DateTime::<Utc>::from_system_time(sample).unwrap_err();
      assert!(failure.source().is_none());
      assert_eq!(
        failure.to_string(),
        format!("system time {sample:?} is outside Chrono's calendar range")
      );
      match failure {
        DateTimeConversionError::SystemTime {
          sample: observed,
        } => assert_eq!(observed, sample),
        unexpected => panic!("expected calendar overflow, got {unexpected:?}"),
      }
    }
  }

  let minimum = DateTime::<Utc>::UNIX_EPOCH
    .signed_duration_since(DateTime::<Utc>::MIN_UTC)
    .to_std()
    .unwrap();
  if let Some(first) = UNIX_EPOCH.checked_sub(minimum) {
    assert_eq!(DateTime::<Utc>::from_system_time(first).unwrap(), DateTime::<Utc>::MIN_UTC);
    if let Some(sample) = first.checked_sub(Duration::from_nanos(1)) {
      match DateTime::<Utc>::from_system_time(sample) {
        Err(DateTimeConversionError::SystemTime {
          sample: observed,
        }) => assert_eq!(observed, sample),
        unexpected => panic!("expected calendar underflow, got {unexpected:?}"),
      }
    }
  }
}

#[test]
fn retains_duration_failure_and_original_sample() {
  let distance = Duration::from_secs(i64::MAX.unsigned_abs());
  let expected_source = TimeDelta::from_std(distance).unwrap_err();
  for sample in [UNIX_EPOCH.checked_add(distance), UNIX_EPOCH.checked_sub(distance)]
    .into_iter()
    .flatten()
  {
    let failure = DateTime::<Utc>::from_system_time(sample).unwrap_err();
    assert_eq!(
      failure.to_string(),
      format!("system time {sample:?} cannot be represented relative to the Unix epoch: {expected_source}"),
    );
    assert_eq!(
      failure
        .source()
        .and_then(|source| source.downcast_ref::<crate::OutOfRangeError>()),
      Some(&expected_source),
    );
    match failure {
      DateTimeConversionError::SystemTimeDuration {
        sample: observed,
        source,
      } => {
        assert_eq!(observed, sample);
        assert_eq!(source, expected_source);
      }
      unexpected => panic!("expected duration overflow, got {unexpected:?}"),
    }
  }
}

#[cfg(feature = "now")]
#[test]
fn acquires_fresh_native_wall_time() -> Result<(), DateTimeConversionError> {
  let before = DateTime::<Utc>::from_system_time(SystemTime::now())?;
  let sampled = Utc::try_now()?;
  let after = DateTime::<Utc>::from_system_time(SystemTime::now())?;
  // Permit a backward clock correction between the bracketing observations.
  assert!((before.min(after)..=before.max(after)).contains(&sampled));
  Ok(())
}
