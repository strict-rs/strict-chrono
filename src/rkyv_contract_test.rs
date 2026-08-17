//! Public archived-type façade compile contracts.

use core::marker::PhantomData;
use core::mem::size_of;

use crate::DateTime;
use crate::FixedOffset;
use crate::IsoWeek;
use crate::Month;
use crate::NaiveDate;
use crate::NaiveDateTime;
use crate::NaiveTime;
use crate::TimeDelta;
use crate::Utc;
use crate::Weekday;

#[test]
fn rkyv_feature_exposes_every_archived_type() {
  let _: PhantomData<crate::rkyv::ArchivedTimeDelta> = PhantomData::<::rkyv::Archived<TimeDelta>>;
  let _: PhantomData<crate::rkyv::ArchivedDuration> = PhantomData::<::rkyv::Archived<TimeDelta>>;
  let _: PhantomData<crate::rkyv::ArchivedNaiveDate> = PhantomData::<::rkyv::Archived<NaiveDate>>;
  let _: PhantomData<crate::rkyv::ArchivedNaiveTime> = PhantomData::<::rkyv::Archived<NaiveTime>>;
  let _: PhantomData<crate::rkyv::ArchivedNaiveDateTime> = PhantomData::<::rkyv::Archived<NaiveDateTime>>;
  let _: PhantomData<crate::rkyv::ArchivedIsoWeek> = PhantomData::<::rkyv::Archived<IsoWeek>>;
  let _: PhantomData<crate::rkyv::ArchivedUtc> = PhantomData::<::rkyv::Archived<Utc>>;
  let _: PhantomData<crate::rkyv::ArchivedFixedOffset> = PhantomData::<::rkyv::Archived<FixedOffset>>;
  let _: PhantomData<crate::rkyv::ArchivedMonth> = PhantomData::<::rkyv::Archived<Month>>;
  let _: PhantomData<crate::rkyv::ArchivedWeekday> = PhantomData::<::rkyv::Archived<Weekday>>;
  let _: PhantomData<crate::rkyv::ArchivedDateTime<Utc>> = PhantomData::<::rkyv::Archived<DateTime<Utc>>>;
  let _: PhantomData<crate::rkyv::ArchivedDateTime<FixedOffset>> = PhantomData::<::rkyv::Archived<DateTime<FixedOffset>>>;
}

#[test]
#[cfg(feature = "rkyv-16")]
fn rkyv_uses_explicit_16_bit_archived_pointers() {
  assert_eq!(size_of::<::rkyv::primitive::ArchivedUsize>(), 2);
}

#[test]
#[cfg(feature = "rkyv-32")]
fn rkyv_uses_explicit_32_bit_archived_pointers() {
  assert_eq!(size_of::<::rkyv::primitive::ArchivedUsize>(), 4);
}

#[test]
#[cfg(feature = "rkyv-64")]
fn rkyv_uses_explicit_64_bit_archived_pointers() {
  assert_eq!(size_of::<::rkyv::primitive::ArchivedUsize>(), 8);
}

#[test]
#[cfg(not(any(feature = "rkyv-16", feature = "rkyv-32", feature = "rkyv-64")))]
fn rkyv_base_uses_32_bit_archived_pointers() {
  assert_eq!(size_of::<::rkyv::primitive::ArchivedUsize>(), 4);
}

#[test]
#[cfg(feature = "clock")]
fn rkyv_feature_exposes_local_archived_types() {
  use crate::Local;

  let _: PhantomData<crate::rkyv::ArchivedLocal> = PhantomData::<::rkyv::Archived<Local>>;
  let _: PhantomData<crate::rkyv::ArchivedDateTime<Local>> = PhantomData::<::rkyv::Archived<DateTime<Local>>>;
}
