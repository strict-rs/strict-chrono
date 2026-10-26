// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The UTC (Coordinated Universal Time) time zone.

use core::fmt;
#[cfg(all(
  feature = "now",
  not(all(
    target_family = "wasm",
    feature = "wasmbind",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
  ))
))]
use std::time::SystemTime;

#[cfg(feature = "rkyv")]
use rkyv::Archive;
#[cfg(feature = "rkyv")]
use rkyv::Deserialize;
#[cfg(feature = "rkyv")]
use rkyv::Serialize;

use super::FixedOffset;
use super::MappedLocalTime;
use super::Offset;
use super::TimeZone;
#[cfg(feature = "now")]
#[allow(deprecated)]
use crate::Date;
#[cfg(feature = "now")]
#[allow(deprecated)]
use crate::DateTime;
#[cfg(feature = "now")]
use crate::DateTimeConversionError;
use crate::naive::NaiveDate;
use crate::naive::NaiveDateTime;

/// The UTC time zone. This is the most efficient time zone when you don't need the local time.
/// It is also used as an offset (which is also a dummy type).
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the UTC struct is the preferred way to construct `DateTime<Utc>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::DateTime;
/// use chrono::TimeZone;
/// use chrono::Utc;
///
/// let dt = DateTime::from_timestamp(61, 0).unwrap();
///
/// assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);
/// assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "rkyv",
  derive(Archive, Deserialize, Serialize),
  rkyv(compare(PartialEq), derive(Clone, Copy, PartialEq, Eq, Debug, Hash))
)]
#[cfg_attr(all(feature = "arbitrary", feature = "std"), derive(arbitrary::Arbitrary))]
pub struct Utc;

#[cfg(feature = "now")]
impl Utc {
  /// Returns a `Date` which corresponds to the current date.
  #[deprecated(
    since = "0.4.23",
    note = "use `Utc::now()` instead, potentially with `.date_naive()`"
  )]
  #[allow(deprecated)]
  #[must_use]
  pub fn today() -> Date<Utc> {
    Utc::now().date()
  }

  /// Returns a `DateTime<Utc>` which corresponds to the current date and time in UTC.
  ///
  /// See also the similar [`Local::now()`] which returns `DateTime<Local>`, i.e. the local date
  /// and time including offset from UTC.
  ///
  /// [`Local::now()`]: crate::Local::now
  ///
  /// # Panics
  ///
  /// Panics when the platform sample is invalid or outside Chrono's range.
  /// Use [`Utc::try_now`] to receive the sample in a typed error instead.
  ///
  /// # Example
  ///
  /// ```
  /// # #![allow(unused_variables)]
  /// # use chrono::{FixedOffset, Utc};
  /// // Current time in UTC
  /// let now_utc = Utc::now();
  ///
  /// // Current date in UTC
  /// let today_utc = now_utc.date_naive();
  ///
  /// // Current time in some timezone (let's use +05:00)
  /// let offset = FixedOffset::east_opt(5 * 60 * 60).unwrap();
  /// let now_with_offset = Utc::now().with_timezone(&offset);
  /// ```
  #[must_use]
  pub fn now() -> DateTime<Utc> {
    Self::try_now().expect("current time outside Chrono's range")
  }

  /// Samples the current UTC wall time, reporting conversion failure explicitly.
  ///
  /// This performs a fresh platform clock read. It is not monotonic: clock
  /// corrections can move successive samples backward. Times before the Unix
  /// epoch are supported. Precision is limited by the platform clock.
  ///
  /// On native platforms this uses [`std::time::SystemTime`]. On JavaScript
  /// WebAssembly targets with `wasmbind`, this uses a JavaScript `Date` with
  /// millisecond precision. It does not recover failures of the host clock
  /// acquisition itself, such as an unsupported target or a host exception.
  ///
  /// # Errors
  ///
  /// Returns [`DateTimeConversionError`] retaining the observed platform sample
  /// when it is invalid or outside Chrono's range.
  ///
  /// ```
  /// # #[cfg(feature = "now")]
  /// # fn main() -> Result<(), chrono::DateTimeConversionError> {
  /// let current = chrono::Utc::try_now()?;
  /// let timestamp = current.timestamp();
  /// # let _ = timestamp;
  /// # Ok(())
  /// # }
  /// # #[cfg(not(feature = "now"))]
  /// # fn main() {}
  /// ```
  #[cfg(not(all(
    target_family = "wasm",
    feature = "wasmbind",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
  )))]
  pub fn try_now() -> Result<DateTime<Utc>, DateTimeConversionError> {
    DateTime::from_system_time(SystemTime::now())
  }

  /// Samples the current UTC wall time from a JavaScript `Date`.
  ///
  /// The sample has millisecond precision and may move backward after a clock
  /// correction. Times before the Unix epoch are supported. Host exceptions
  /// during acquisition are not converted to Rust errors.
  ///
  /// # Errors
  ///
  /// Returns [`DateTimeConversionError::JsDate`] retaining the sample and its
  /// observed milliseconds if it is invalid or outside Chrono's range.
  #[cfg(all(
    target_family = "wasm",
    feature = "wasmbind",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
  ))]
  pub fn try_now() -> Result<DateTime<Utc>, DateTimeConversionError> {
    DateTime::from_js_date(js_sys::Date::new_0())
  }
}

impl TimeZone for Utc {
  type Offset = Utc;

  fn from_offset(_state: &Utc) -> Utc {
    Utc
  }

  fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<Utc> {
    MappedLocalTime::Single(Utc)
  }
  fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<Utc> {
    MappedLocalTime::Single(Utc)
  }

  fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc {
    Utc
  }
  fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc {
    Utc
  }
}

impl Offset for Utc {
  fn fix(&self) -> FixedOffset {
    FixedOffset::east_opt(0).unwrap()
  }
}

impl fmt::Debug for Utc {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Z")
  }
}

impl fmt::Display for Utc {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "UTC")
  }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Utc {
  fn format(&self, fmt: defmt::Formatter) {
    defmt::write!(fmt, "Z");
  }
}
