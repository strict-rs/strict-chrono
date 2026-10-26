//! Checked conversion of platform wall-clock samples.

use core::fmt;
#[cfg(feature = "std")]
use std::time::SystemTime;
#[cfg(feature = "std")]
use std::time::UNIX_EPOCH;

use crate::DateTime;
#[cfg(feature = "std")]
use crate::OutOfRangeError;
#[cfg(feature = "std")]
use crate::TimeDelta;
use crate::Utc;

/// A platform clock sample that cannot be represented as a [`DateTime<Utc>`].
///
/// Each variant retains the original sample. A time before the Unix epoch is
/// supported and is not itself an error.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DateTimeConversionError {
  /// The sample's distance from the Unix epoch exceeds [`TimeDelta`]'s range.
  #[cfg(feature = "std")]
  SystemTimeDuration {
    /// The original platform sample.
    sample: SystemTime,
    /// The native duration conversion failure.
    source: OutOfRangeError,
  },
  /// The sample is outside [`DateTime::MIN_UTC`] through [`DateTime::MAX_UTC`].
  #[cfg(feature = "std")]
  SystemTime {
    /// The original platform sample.
    sample: SystemTime,
  },
  /// The JavaScript date is invalid or outside Chrono's supported calendar.
  #[cfg(all(
    target_family = "wasm",
    feature = "wasmbind",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
  ))]
  JsDate {
    /// The original JavaScript object, which callers may inspect or recover.
    sample:       js_sys::Date,
    /// The milliseconds observed during conversion, retained even if the object is later mutated.
    milliseconds: f64,
  },
}

impl fmt::Display for DateTimeConversionError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      #[cfg(feature = "std")]
      Self::SystemTimeDuration {
        sample,
        source,
      } => {
        write!(
          formatter,
          "system time {sample:?} cannot be represented relative to the Unix epoch: {source}"
        )
      }
      #[cfg(feature = "std")]
      Self::SystemTime {
        sample,
      } => write!(formatter, "system time {sample:?} is outside Chrono's calendar range"),
      #[cfg(all(
        target_family = "wasm",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
      ))]
      Self::JsDate {
        milliseconds, ..
      } => {
        write!(
          formatter,
          "JavaScript date with timestamp {milliseconds} ms is invalid or outside Chrono's calendar range"
        )
      }
    }
  }
}

#[cfg(any(feature = "std", feature = "core-error"))]
impl core::error::Error for DateTimeConversionError {
  fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
    match self {
      #[cfg(feature = "std")]
      Self::SystemTimeDuration {
        source, ..
      } => Some(source),
      #[cfg(feature = "std")]
      Self::SystemTime {
        ..
      } => None,
      #[cfg(all(
        target_family = "wasm",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
      ))]
      Self::JsDate {
        ..
      } => None,
    }
  }
}

impl DateTime<Utc> {
  /// Converts a platform wall-clock sample without panicking on an out-of-range time.
  ///
  /// This does not acquire a new sample. Times on either side of the Unix epoch
  /// retain their full nanosecond precision. The platform's range and resolution
  /// still apply to the supplied [`SystemTime`].
  ///
  /// # Errors
  ///
  /// Returns [`DateTimeConversionError`] containing `sample` when its distance
  /// from the epoch or its calendar date is outside Chrono's supported range.
  ///
  /// ```
  /// # #[cfg(feature = "std")]
  /// # fn main() -> Result<(), chrono::DateTimeConversionError> {
  /// use std::time::UNIX_EPOCH;
  ///
  /// use chrono::DateTime;
  /// use chrono::Utc;
  ///
  /// let instant = DateTime::<Utc>::from_system_time(UNIX_EPOCH)?;
  /// assert_eq!(instant, DateTime::UNIX_EPOCH);
  /// # Ok(())
  /// # }
  /// # #[cfg(not(feature = "std"))]
  /// # fn main() {}
  /// ```
  #[cfg(feature = "std")]
  pub fn from_system_time(sample: SystemTime) -> Result<Self, DateTimeConversionError> {
    let (duration, before_epoch) = match sample.duration_since(UNIX_EPOCH) {
      Ok(duration) => (duration, false),
      Err(before) => (before.duration(), true),
    };
    let delta = TimeDelta::from_std(duration).map_err(|source| DateTimeConversionError::SystemTimeDuration {
      sample,
      source,
    })?;
    let datetime = if before_epoch {
      Self::UNIX_EPOCH.checked_sub_signed(delta)
    } else {
      Self::UNIX_EPOCH.checked_add_signed(delta)
    };
    datetime.ok_or(DateTimeConversionError::SystemTime {
      sample,
    })
  }

  /// Converts a JavaScript date without treating an invalid date as the Unix epoch.
  ///
  /// JavaScript dates have integer millisecond precision. A valid date before
  /// the Unix epoch retains its sign. The date is sampled once; errors preserve
  /// both the object and the milliseconds observed at that moment.
  ///
  /// # Errors
  ///
  /// Returns [`DateTimeConversionError::JsDate`] for an invalid JavaScript date
  /// (`NaN`) or a date outside Chrono's calendar range.
  #[cfg(all(
    target_family = "wasm",
    feature = "wasmbind",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
  ))]
  pub fn from_js_date(sample: js_sys::Date) -> Result<Self, DateTimeConversionError> {
    use num_traits::ToPrimitive;

    let milliseconds = sample.get_time();
    milliseconds
      .to_i64()
      .and_then(Self::from_timestamp_millis)
      .ok_or(DateTimeConversionError::JsDate {
        sample,
        milliseconds,
      })
  }
}

#[cfg(all(test, feature = "std", not(target_family = "wasm")))]
mod tests;
