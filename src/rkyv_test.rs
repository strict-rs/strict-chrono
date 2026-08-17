//! Shared allocation-free archive helpers and integration-contract tests.

use core::mem::MaybeUninit;

use rkyv::Archive;
use rkyv::Deserialize;
use rkyv::Serialize;
use rkyv::api::low::LowSerializer;
use rkyv::api::low::LowValidator;
use rkyv::api::low::from_bytes;
use rkyv::api::low::to_bytes_in_with_alloc;
use rkyv::bytecheck::CheckBytes;
use rkyv::de::pooling::Unpool;
use rkyv::rancor::Failure;
use rkyv::rancor::Strategy;
use rkyv::ser::allocator::SubAllocator;
use rkyv::ser::writer::Buffer;
use rkyv::util::Align;

/// Fixed storage available to the archive writer and scratch allocator.
const ARCHIVE_CAPACITY: usize = 256;

/// Archives and checked-decodes one value without enabling `rkyv/alloc`.
pub(crate) fn roundtrip<T>(value: &T) -> Result<T, Failure>
where
  T: Archive,
  for<'output, 'scratch> T: Serialize<LowSerializer<Buffer<'output>, SubAllocator<'scratch>, Failure>>,
  T::Archived: for<'bytes> CheckBytes<LowValidator<'bytes, Failure>> + Deserialize<T, Strategy<Unpool, Failure>>,
{
  let mut output = Align([MaybeUninit::<u8>::uninit(); ARCHIVE_CAPACITY]);
  let mut scratch = [MaybeUninit::<u8>::uninit(); ARCHIVE_CAPACITY];
  let bytes = to_bytes_in_with_alloc::<_, _, Failure>(value, Buffer::from(&mut *output), SubAllocator::new(&mut scratch))?;
  from_bytes::<T, Failure>(&bytes)
}

/// Archives one value and lets a test inspect its initialized byte slice.
pub(crate) fn inspect_archive<T, R>(value: &T, inspect: impl FnOnce(&[u8]) -> R) -> Result<R, Failure>
where
  for<'output, 'scratch> T: Serialize<LowSerializer<Buffer<'output>, SubAllocator<'scratch>, Failure>>,
{
  let mut output = Align([MaybeUninit::<u8>::uninit(); ARCHIVE_CAPACITY]);
  let mut scratch = [MaybeUninit::<u8>::uninit(); ARCHIVE_CAPACITY];
  let bytes = to_bytes_in_with_alloc::<_, _, Failure>(value, Buffer::from(&mut *output), SubAllocator::new(&mut scratch))?;
  Ok(inspect(&bytes))
}

#[cfg(test)]
mod tests {
  use super::inspect_archive;
  use super::roundtrip;
  use crate::DateTime;
  use crate::FixedOffset;
  use crate::Month;
  use crate::NaiveDate;
  use crate::NaiveDateTime;
  use crate::NaiveTime;
  use crate::TimeDelta;
  use crate::TimeZone;
  use crate::Utc;
  use crate::Weekday;

  #[test]
  fn rkyv_roundtrips_timezone_aware_values() {
    let naive = NaiveDate::from_ymd_opt(2024, 2, 29)
      .unwrap()
      .and_time(NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999).unwrap());
    let utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
    assert_eq!(roundtrip(&utc).unwrap(), utc);

    let offset = FixedOffset::east_opt(5 * 60 * 60 + 30 * 60).unwrap();
    let fixed = offset.from_local_datetime(&naive).single().unwrap();
    assert_eq!(roundtrip(&fixed).unwrap(), fixed);
  }

  #[test]
  #[cfg(feature = "clock")]
  fn rkyv_roundtrips_local_datetime() {
    use crate::Local;

    let local = Local.with_ymd_and_hms(2024, 2, 29, 12, 34, 56).earliest().unwrap();
    assert_eq!(roundtrip(&local).unwrap(), local);
  }

  #[test]
  fn rkyv_rejects_corrupt_enum_discriminant() {
    inspect_archive(&Month::January, |bytes| {
      let mut corrupt = [0_u8; 1];
      corrupt.copy_from_slice(bytes);
      corrupt[0] = u8::MAX;
      assert!(rkyv::api::low::from_bytes::<Month, rkyv::rancor::Failure>(&corrupt).is_err());
    })
    .unwrap();
  }

  #[test]
  fn rkyv_rejects_truncated_archive() {
    inspect_archive(&Weekday::Mon, |bytes| {
      assert!(rkyv::api::low::from_bytes::<Weekday, rkyv::rancor::Failure>(&bytes[..0]).is_err());
    })
    .unwrap();
  }

  #[test]
  fn rkyv_roundtrips_representative_naive_datetime() {
    let value = NaiveDateTime::new(
      NaiveDate::from_ymd_opt(2000, 2, 29).unwrap(),
      NaiveTime::from_hms_milli_opt(6, 7, 8, 901).unwrap(),
    );
    assert_eq!(roundtrip(&value).unwrap(), value);
  }

  #[test]
  fn rkyv_roundtrips_duration_and_enum_boundaries() {
    assert_eq!(roundtrip(&TimeDelta::MIN).unwrap(), TimeDelta::MIN);
    assert_eq!(roundtrip(&TimeDelta::MAX).unwrap(), TimeDelta::MAX);
    assert_eq!(roundtrip(&Month::January).unwrap(), Month::January);
    assert_eq!(roundtrip(&Month::December).unwrap(), Month::December);
    assert_eq!(roundtrip(&Weekday::Mon).unwrap(), Weekday::Mon);
    assert_eq!(roundtrip(&Weekday::Sun).unwrap(), Weekday::Sun);
  }

  #[test]
  fn rkyv_roundtrips_offset_boundaries_and_zst() {
    let minimum = FixedOffset::west_opt(86_399).unwrap();
    let zero = FixedOffset::east_opt(0).unwrap();
    let maximum = FixedOffset::east_opt(86_399).unwrap();

    assert_eq!(roundtrip(&minimum).unwrap(), minimum);
    assert_eq!(roundtrip(&zero).unwrap(), zero);
    assert_eq!(roundtrip(&maximum).unwrap(), maximum);
    assert_eq!(roundtrip(&Utc).unwrap(), Utc);
  }

  #[test]
  fn rkyv_roundtrips_datetime_boundaries() {
    let zero = FixedOffset::east_opt(0).unwrap();
    let utc_min = DateTime::<Utc>::MIN_UTC;
    let utc_max = DateTime::<Utc>::MAX_UTC;
    let fixed_min = DateTime::<FixedOffset>::from_naive_utc_and_offset(NaiveDateTime::MIN, zero);
    let fixed_max = DateTime::<FixedOffset>::from_naive_utc_and_offset(NaiveDateTime::MAX, zero);

    assert_eq!(roundtrip(&utc_min).unwrap(), utc_min);
    assert_eq!(roundtrip(&utc_max).unwrap(), utc_max);
    assert_eq!(roundtrip(&fixed_min).unwrap(), fixed_min);
    assert_eq!(roundtrip(&fixed_max).unwrap(), fixed_max);
  }

  #[test]
  #[cfg(feature = "clock")]
  fn rkyv_roundtrips_local_datetime_boundaries() {
    use crate::Local;

    let zero = FixedOffset::east_opt(0).unwrap();
    let minimum = DateTime::<Local>::from_naive_utc_and_offset(NaiveDateTime::MIN, zero);
    let maximum = DateTime::<Local>::from_naive_utc_and_offset(NaiveDateTime::MAX, zero);

    assert_eq!(roundtrip(&minimum).unwrap(), minimum);
    assert_eq!(roundtrip(&maximum).unwrap(), maximum);
  }
}
