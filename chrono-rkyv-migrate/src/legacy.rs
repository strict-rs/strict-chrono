//! Frozen `rkyv` `0.7` representations and checked chrono conversions.

#![allow(
  deprecated,
  reason = "this isolated compatibility module decodes historical chrono Date archives"
)]

use core::num::NonZeroI32;

use chrono::Date;
use chrono::DateTime;
use chrono::Datelike;
use chrono::FixedOffset;
use chrono::IsoWeek;
use chrono::Local;
use chrono::Month;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeDelta;
use chrono::Utc;
use chrono::Weekday;
use rkyv07::Archive;
use rkyv07::Deserialize;
use rkyv07::Serialize;

use crate::LegacyDecode;
use crate::MigrationError;

/// Frozen legacy representation of `TimeDelta`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyTimeDelta {
  /// Whole seconds.
  secs:  i64,
  /// Nanoseconds within the represented second.
  nanos: i32,
}

/// Frozen legacy representation of `NaiveDate`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyNaiveDate {
  /// Packed year, ordinal, and year flags.
  yof: NonZeroI32,
}

/// Frozen legacy representation of `NaiveTime`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyNaiveTime {
  /// Seconds since midnight.
  secs: u32,
  /// Fractional nanoseconds, including leap-second encoding.
  frac: u32,
}

/// Frozen legacy representation of `NaiveDateTime`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyNaiveDateTime {
  /// Calendar date.
  date: LegacyNaiveDate,
  /// Wall-clock time.
  time: LegacyNaiveTime,
}

/// Frozen legacy representation of `IsoWeek`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyIsoWeek {
  /// Packed ISO year, week, and year flags.
  ywf: i32,
}

/// Frozen legacy representation of `Utc`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyUtc;

/// Frozen legacy representation of `Local`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyLocal;

/// Frozen legacy representation of `FixedOffset`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyFixedOffset {
  /// Local seconds east of UTC.
  local_minus_utc: i32,
}

/// Frozen legacy representation of `Month`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
enum LegacyMonth {
  /// January.
  January   = 0,
  /// February.
  February  = 1,
  /// March.
  March     = 2,
  /// April.
  April     = 3,
  /// May.
  May       = 4,
  /// June.
  June      = 5,
  /// July.
  July      = 6,
  /// August.
  August    = 7,
  /// September.
  September = 8,
  /// October.
  October   = 9,
  /// November.
  November  = 10,
  /// December.
  December  = 11,
}

/// Frozen legacy representation of `Weekday`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
enum LegacyWeekday {
  /// Monday.
  Mon = 0,
  /// Tuesday.
  Tue = 1,
  /// Wednesday.
  Wed = 2,
  /// Thursday.
  Thu = 3,
  /// Friday.
  Fri = 4,
  /// Saturday.
  Sat = 5,
  /// Sunday.
  Sun = 6,
}

/// Frozen legacy representation of `DateTime<Utc>`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyDateTimeUtc {
  /// UTC naive datetime.
  datetime: LegacyNaiveDateTime,
  /// UTC marker.
  offset:   LegacyUtc,
}

/// Frozen legacy representation shared by `DateTime<FixedOffset>` and
/// `DateTime<Local>`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyDateTimeFixed {
  /// UTC naive datetime.
  datetime: LegacyNaiveDateTime,
  /// Fixed offset active at that instant.
  offset:   LegacyFixedOffset,
}

/// Frozen legacy representation of `Date<Utc>`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyDateUtc {
  /// UTC date.
  date:   LegacyNaiveDate,
  /// UTC marker.
  offset: LegacyUtc,
}

/// Frozen legacy representation shared by `Date<FixedOffset>` and
/// `Date<Local>`.
#[derive(Clone, Copy, Archive, Deserialize, Serialize)]
#[archive(check_bytes, crate = "rkyv07")]
struct LegacyDateFixed {
  /// UTC date.
  date:   LegacyNaiveDate,
  /// Fixed offset active for the date.
  offset: LegacyFixedOffset,
}

/// Frozen current representation of `TimeDelta`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentTimeDelta {
  /// Whole seconds.
  secs:  i64,
  /// Nanoseconds within the represented second.
  nanos: i32,
}

/// Frozen current representation of `NaiveDate`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentNaiveDate {
  /// Packed year, ordinal, and year flags.
  yof: NonZeroI32,
}

/// Frozen current representation of `NaiveTime`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentNaiveTime {
  /// Seconds since midnight.
  secs: u32,
  /// Fractional nanoseconds, including leap-second encoding.
  frac: u32,
}

/// Frozen current representation of `NaiveDateTime`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentNaiveDateTime {
  /// Calendar date.
  date: CurrentNaiveDate,
  /// Wall-clock time.
  time: CurrentNaiveTime,
}

/// Frozen current representation of `IsoWeek`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentIsoWeek {
  /// Packed ISO year, week, and year flags.
  ywf: i32,
}

/// Frozen current representation of `FixedOffset`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentFixedOffset {
  /// Local seconds east of UTC.
  local_minus_utc: i32,
}

/// Frozen current representation of `DateTime<Utc>`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentDateTimeUtc {
  /// UTC naive datetime.
  datetime: CurrentNaiveDateTime,
  /// UTC marker.
  offset:   Utc,
}

/// Frozen current representation shared by `DateTime<FixedOffset>` and
/// `DateTime<Local>`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentDateTimeFixed {
  /// UTC naive datetime.
  datetime: CurrentNaiveDateTime,
  /// Fixed offset active at that instant.
  offset:   CurrentFixedOffset,
}

/// Frozen current representation of `Date<Utc>`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentDateUtc {
  /// UTC date.
  date:   CurrentNaiveDate,
  /// UTC marker.
  offset: Utc,
}

/// Frozen current representation shared by `Date<FixedOffset>` and
/// `Date<Local>`.
#[derive(rkyv::Archive, rkyv::Deserialize)]
struct CurrentDateFixed {
  /// UTC date.
  date:   CurrentNaiveDate,
  /// Fixed offset active for the date.
  offset: CurrentFixedOffset,
}

/// Formats one structurally invalid `rkyv` `0.7` archive stage.
fn unreadable_legacy(error: impl core::fmt::Display) -> MigrationError {
  MigrationError::UnreadableInput {
    legacy:  error.to_string(),
    current: "current decoder was not run".to_owned(),
  }
}

/// Reports one invalid value recovered from a structurally valid archive.
fn invalid(detail: impl Into<String>) -> MigrationError {
  MigrationError::InvalidLegacyValue {
    detail: detail.into()
  }
}

/// Reports an invalid current archive or logical value.
fn invalid_current(detail: impl Into<String>) -> MigrationError {
  MigrationError::InvalidCurrentValue {
    detail: detail.into()
  }
}

/// Owned-detail constructor used by representation-neutral converters.
fn invalid_legacy_detail(detail: String) -> MigrationError {
  invalid(detail)
}

/// Owned-detail constructor used by representation-neutral converters.
fn invalid_current_detail(detail: String) -> MigrationError {
  invalid_current(detail)
}

/// Checked-decodes one frozen current representation.
fn decode_current<T>(bytes: &[u8]) -> Result<T, MigrationError>
where
  T: rkyv::Archive,
  T::Archived: for<'bytes> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'bytes, rancor::Error>>
    + rkyv::Deserialize<T, rkyv::rancor::Strategy<rkyv::de::Pool, rancor::Error>>,
{
  rkyv::from_bytes::<T, rancor::Error>(bytes).map_err(|error| invalid_current(error.to_string()))
}

/// Returns whether the proleptic Gregorian year is a leap year.
fn is_leap_year(year: i32) -> bool {
  year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

/// Reconstructs chrono's packed `LWWW` year flags through Gregorian arithmetic.
fn expected_year_flags(year: i32) -> u8 {
  let prior_year = i64::from(year) - 1;
  let days_before_year = prior_year * 365 + prior_year.div_euclid(4) - prior_year.div_euclid(100) + prior_year.div_euclid(400);
  let prior_weekday = u8::try_from((days_before_year - 1).rem_euclid(7)).unwrap_or_default();
  let weekday_flags = if prior_weekday == 0 { 7 } else { prior_weekday };
  let common_year_flag = if is_leap_year(year) { 0 } else { 8 };
  common_year_flag | weekday_flags
}

/// Converts one packed date through chrono's checked ordinal constructor.
fn convert_date_packed(packed: i32, invalid_value: fn(String) -> MigrationError) -> Result<NaiveDate, MigrationError> {
  let year = packed >> 13;
  let ordinal = u32::try_from((packed & 0x1ff0) >> 4).map_err(|_| invalid_value("packed date contains a negative ordinal".to_owned()))?;
  let flags = u8::try_from(packed & 0x0f).map_err(|_| invalid_value("packed date contains invalid year flags".to_owned()))?;
  if flags != expected_year_flags(year) {
    return Err(invalid_value(format!("packed date year flags {flags:#x} do not match year {year}")));
  }
  NaiveDate::from_yo_opt(year, ordinal)
    .ok_or_else(|| invalid_value(format!("packed date contains invalid year {year} and ordinal {ordinal}")))
}

/// Converts a packed legacy date.
fn convert_date(value: LegacyNaiveDate) -> Result<NaiveDate, MigrationError> {
  convert_date_packed(value.yof.get(), invalid_legacy_detail)
}

/// Converts seconds and fractional nanoseconds through chrono's checked
/// constructor.
fn convert_time_parts(secs: u32, frac: u32, invalid_value: fn(String) -> MigrationError) -> Result<NaiveTime, MigrationError> {
  NaiveTime::from_num_seconds_from_midnight_opt(secs, frac)
    .ok_or_else(|| invalid_value(format!("time contains invalid seconds {secs} and nanoseconds {frac}")))
}

/// Converts one frozen legacy time.
fn convert_time(value: LegacyNaiveTime) -> Result<NaiveTime, MigrationError> {
  convert_time_parts(value.secs, value.frac, invalid_legacy_detail)
}

/// Converts a frozen naive datetime.
fn convert_naive_datetime(value: LegacyNaiveDateTime) -> Result<NaiveDateTime, MigrationError> {
  Ok(NaiveDateTime::new(convert_date(value.date)?, convert_time(value.time)?))
}

/// Converts an offset through chrono's checked constructor.
fn convert_fixed_offset_seconds(local_minus_utc: i32, invalid_value: fn(String) -> MigrationError) -> Result<FixedOffset, MigrationError> {
  FixedOffset::east_opt(local_minus_utc)
    .ok_or_else(|| invalid_value(format!("fixed offset {local_minus_utc} is outside chrono's supported range")))
}

/// Converts one frozen legacy offset.
fn convert_fixed_offset(value: LegacyFixedOffset) -> Result<FixedOffset, MigrationError> {
  convert_fixed_offset_seconds(value.local_minus_utc, invalid_legacy_detail)
}

/// Converts a packed ISO week, including chrono's calendar boundaries.
fn convert_iso_week_packed(ywf: i32, invalid_value: fn(String) -> MigrationError) -> Result<IsoWeek, MigrationError> {
  let year = ywf >> 10;
  let week = u32::try_from((ywf >> 4) & 0x3f).map_err(|_| invalid_value("packed ISO week contains a negative week number".to_owned()))?;
  let flags = u8::try_from(ywf & 0x0f).map_err(|_| invalid_value("packed ISO week contains invalid year flags".to_owned()))?;
  if flags != expected_year_flags(year) {
    return Err(invalid_value(format!("packed ISO week flags {flags:#x} do not match year {year}")));
  }

  [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
  ]
  .into_iter()
  .find_map(|weekday| NaiveDate::from_isoywd_opt(year, week, weekday))
  .map(|date| date.iso_week())
  .filter(|iso_week| iso_week.year() == year && iso_week.week() == week)
  .ok_or_else(|| invalid_value(format!("packed ISO week contains invalid year {year} and week {week}")))
}

/// Converts one frozen legacy ISO week.
fn convert_iso_week(value: LegacyIsoWeek) -> Result<IsoWeek, MigrationError> {
  convert_iso_week_packed(value.ywf, invalid_legacy_detail)
}

/// Converts a frozen current date.
fn convert_current_date(value: CurrentNaiveDate) -> Result<NaiveDate, MigrationError> {
  convert_date_packed(value.yof.get(), invalid_current_detail)
}

/// Converts a frozen current time.
fn convert_current_time(value: CurrentNaiveTime) -> Result<NaiveTime, MigrationError> {
  convert_time_parts(value.secs, value.frac, invalid_current_detail)
}

/// Converts a frozen current naive datetime.
fn convert_current_naive_datetime(value: CurrentNaiveDateTime) -> Result<NaiveDateTime, MigrationError> {
  Ok(NaiveDateTime::new(
    convert_current_date(value.date)?,
    convert_current_time(value.time)?,
  ))
}

/// Converts a frozen current offset.
fn convert_current_fixed_offset(value: CurrentFixedOffset) -> Result<FixedOffset, MigrationError> {
  convert_fixed_offset_seconds(value.local_minus_utc, invalid_current_detail)
}

/// Converts raw duration fields through chrono's checked constructor.
fn convert_duration_parts(secs: i64, nanos: i32, invalid_value: fn(String) -> MigrationError) -> Result<TimeDelta, MigrationError> {
  let nanos = u32::try_from(nanos).map_err(|_| invalid_value(format!("duration nanoseconds {nanos} are negative")))?;
  TimeDelta::new(secs, nanos).ok_or_else(|| {
    invalid_value(format!(
      "duration seconds {secs} and nanoseconds {nanos} are outside chrono's supported range"
    ))
  })
}

impl LegacyDecode for TimeDelta {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyTimeDelta>(bytes).map_err(unreadable_legacy)?;
    convert_duration_parts(value.secs, value.nanos, invalid_legacy_detail)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentTimeDelta>(bytes)?;
    convert_duration_parts(value.secs, value.nanos, invalid_current_detail)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for NaiveDate {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyNaiveDate>(bytes).map_err(unreadable_legacy)?;
    convert_date(value)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    convert_current_date(decode_current::<CurrentNaiveDate>(bytes)?)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for NaiveTime {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyNaiveTime>(bytes).map_err(unreadable_legacy)?;
    convert_time(value)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    convert_current_time(decode_current::<CurrentNaiveTime>(bytes)?)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for NaiveDateTime {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyNaiveDateTime>(bytes).map_err(unreadable_legacy)?;
    convert_naive_datetime(value)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    convert_current_naive_datetime(decode_current::<CurrentNaiveDateTime>(bytes)?)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for IsoWeek {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyIsoWeek>(bytes).map_err(unreadable_legacy)?;
    convert_iso_week(value)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentIsoWeek>(bytes)?;
    convert_iso_week_packed(value.ywf, invalid_current_detail)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Utc {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    rkyv07::from_bytes::<LegacyUtc>(bytes).map_err(unreadable_legacy)?;
    Ok(Utc)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Local {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    rkyv07::from_bytes::<LegacyLocal>(bytes).map_err(unreadable_legacy)?;
    Ok(Local)
  }

  fn logically_equals(&self, _other: &Self) -> bool {
    true
  }
}

impl LegacyDecode for FixedOffset {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyFixedOffset>(bytes).map_err(unreadable_legacy)?;
    convert_fixed_offset(value)
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    convert_current_fixed_offset(decode_current::<CurrentFixedOffset>(bytes)?)
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Month {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyMonth>(bytes).map_err(unreadable_legacy)?;
    Ok(match value {
      LegacyMonth::January => Month::January,
      LegacyMonth::February => Month::February,
      LegacyMonth::March => Month::March,
      LegacyMonth::April => Month::April,
      LegacyMonth::May => Month::May,
      LegacyMonth::June => Month::June,
      LegacyMonth::July => Month::July,
      LegacyMonth::August => Month::August,
      LegacyMonth::September => Month::September,
      LegacyMonth::October => Month::October,
      LegacyMonth::November => Month::November,
      LegacyMonth::December => Month::December,
    })
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Weekday {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyWeekday>(bytes).map_err(unreadable_legacy)?;
    Ok(match value {
      LegacyWeekday::Mon => Weekday::Mon,
      LegacyWeekday::Tue => Weekday::Tue,
      LegacyWeekday::Wed => Weekday::Wed,
      LegacyWeekday::Thu => Weekday::Thu,
      LegacyWeekday::Fri => Weekday::Fri,
      LegacyWeekday::Sat => Weekday::Sat,
      LegacyWeekday::Sun => Weekday::Sun,
    })
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for DateTime<Utc> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateTimeUtc>(bytes).map_err(unreadable_legacy)?;
    Ok(DateTime::from_naive_utc_and_offset(convert_naive_datetime(value.datetime)?, Utc))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateTimeUtc>(bytes)?;
    Ok(DateTime::from_naive_utc_and_offset(
      convert_current_naive_datetime(value.datetime)?,
      value.offset,
    ))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for DateTime<FixedOffset> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateTimeFixed>(bytes).map_err(unreadable_legacy)?;
    Ok(DateTime::from_naive_utc_and_offset(
      convert_naive_datetime(value.datetime)?,
      convert_fixed_offset(value.offset)?,
    ))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateTimeFixed>(bytes)?;
    Ok(DateTime::from_naive_utc_and_offset(
      convert_current_naive_datetime(value.datetime)?,
      convert_current_fixed_offset(value.offset)?,
    ))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for DateTime<Local> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateTimeFixed>(bytes).map_err(unreadable_legacy)?;
    Ok(DateTime::from_naive_utc_and_offset(
      convert_naive_datetime(value.datetime)?,
      convert_fixed_offset(value.offset)?,
    ))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateTimeFixed>(bytes)?;
    Ok(DateTime::from_naive_utc_and_offset(
      convert_current_naive_datetime(value.datetime)?,
      convert_current_fixed_offset(value.offset)?,
    ))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Date<Utc> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateUtc>(bytes).map_err(unreadable_legacy)?;
    Ok(Date::from_utc(convert_date(value.date)?, Utc))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateUtc>(bytes)?;
    Ok(Date::from_utc(convert_current_date(value.date)?, value.offset))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Date<FixedOffset> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateFixed>(bytes).map_err(unreadable_legacy)?;
    Ok(Date::from_utc(convert_date(value.date)?, convert_fixed_offset(value.offset)?))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateFixed>(bytes)?;
    Ok(Date::from_utc(
      convert_current_date(value.date)?,
      convert_current_fixed_offset(value.offset)?,
    ))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

impl LegacyDecode for Date<Local> {
  fn decode_legacy(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = rkyv07::from_bytes::<LegacyDateFixed>(bytes).map_err(unreadable_legacy)?;
    Ok(Date::from_utc(convert_date(value.date)?, convert_fixed_offset(value.offset)?))
  }

  fn decode_current(bytes: &[u8]) -> Result<Self, MigrationError> {
    let value = decode_current::<CurrentDateFixed>(bytes)?;
    Ok(Date::from_utc(
      convert_current_date(value.date)?,
      convert_current_fixed_offset(value.offset)?,
    ))
  }

  fn logically_equals(&self, other: &Self) -> bool {
    self == other
  }
}

#[cfg(test)]
mod tests {
  use core::num::NonZeroI32;

  use chrono::Date;
  use chrono::DateTime;
  use chrono::Datelike;
  use chrono::FixedOffset;
  use chrono::Local;
  use chrono::NaiveDateTime;
  use chrono::Timelike;
  use chrono::Utc;

  use super::LegacyDateFixed;
  use super::LegacyDateTimeFixed;
  use super::LegacyDateTimeUtc;
  use super::LegacyDateUtc;
  use super::LegacyFixedOffset;
  use super::LegacyIsoWeek;
  use super::LegacyLocal;
  use super::LegacyMonth;
  use super::LegacyNaiveDate;
  use super::LegacyNaiveDateTime;
  use super::LegacyNaiveTime;
  use super::LegacyTimeDelta;
  use super::LegacyUtc;
  use super::LegacyWeekday;
  use super::convert_date;
  use super::convert_iso_week;
  use super::convert_time;
  use super::expected_year_flags;
  use crate::DetectedSource;
  use crate::SourceHint;
  use crate::migrate;

  /// Serializes one frozen mirror and verifies forced legacy migration.
  macro_rules! assert_migrates {
    ($mirror:expr, $target:ty, $expected:expr) => {{
      let bytes = rkyv07::to_bytes::<_, 256>(&$mirror).unwrap();
      let migrated = migrate::<$target>(&bytes, SourceHint::Rkyv0_7).unwrap();
      assert!(crate::LegacyDecode::logically_equals(&migrated.value, &$expected));
      assert_eq!(migrated.detected_source, DetectedSource::Rkyv0_7);
      let repeated = migrate::<$target>(&migrated.canonical_bytes, SourceHint::Rkyv0_8).unwrap();
      assert!(crate::LegacyDecode::logically_equals(&repeated.value, &$expected));
      assert_eq!(repeated.canonical_bytes.as_slice(), migrated.canonical_bytes.as_slice());
    }};
  }

  #[test]
  fn reconstructs_known_year_flags() {
    assert_eq!(expected_year_flags(0), 0o04);
    assert_eq!(expected_year_flags(1), 0o16);
    assert_eq!(expected_year_flags(2000), 0o04);
    assert_eq!(expected_year_flags(2024), 0o06);
  }

  #[test]
  fn converts_packed_date_through_public_constructor() {
    let packed = (2024 << 13) | (60 << 4) | i32::from(expected_year_flags(2024));
    let date = convert_date(LegacyNaiveDate {
      yof: NonZeroI32::new(packed).unwrap(),
    })
    .unwrap();
    assert_eq!((date.year(), date.ordinal()), (2024, 60));
  }

  #[test]
  fn rejects_incorrect_packed_date_flags() {
    let packed = (2024 << 13) | (60 << 4) | 0x0f;
    assert!(
      convert_date(LegacyNaiveDate {
        yof: NonZeroI32::new(packed).unwrap(),
      })
      .is_err()
    );
  }

  #[test]
  fn preserves_leap_second_time_encoding() {
    let time = convert_time(LegacyNaiveTime {
      secs: 86_399,
      frac: 1_123_456_789,
    })
    .unwrap();
    assert_eq!(time.num_seconds_from_midnight(), 86_399);
    assert_eq!(time.nanosecond(), 1_123_456_789);
  }

  #[test]
  fn rejects_invalid_time_encoding() {
    assert!(
      convert_time(LegacyNaiveTime {
        secs: 86_400, frac: 0
      })
      .is_err()
    );
  }

  #[test]
  fn auto_rejects_invalid_packed_date() {
    let packed = (2024 << 13) | (60 << 4) | 0x0f;
    let bytes = rkyv07::to_bytes::<_, 256>(&LegacyNaiveDate {
      yof: NonZeroI32::new(packed).unwrap(),
    })
    .unwrap();
    assert!(matches!(
      migrate::<chrono::NaiveDate>(&bytes, SourceHint::Auto),
      Err(crate::MigrationError::UnreadableInput { .. })
    ));
  }

  #[test]
  fn auto_rejects_invalid_time() {
    let bytes = rkyv07::to_bytes::<_, 256>(&LegacyNaiveTime {
      secs: 86_400, frac: 0
    })
    .unwrap();
    assert!(matches!(
      migrate::<chrono::NaiveTime>(&bytes, SourceHint::Auto),
      Err(crate::MigrationError::UnreadableInput { .. })
    ));
  }

  #[test]
  fn auto_rejects_invalid_offset() {
    let bytes = rkyv07::to_bytes::<_, 256>(&LegacyFixedOffset {
      local_minus_utc: 86_400
    })
    .unwrap();
    assert!(matches!(
      migrate::<chrono::FixedOffset>(&bytes, SourceHint::Auto),
      Err(crate::MigrationError::UnreadableInput { .. })
    ));
  }

  #[test]
  fn auto_rejects_invalid_enum_discriminant() {
    let mut bytes = rkyv07::to_bytes::<_, 256>(&LegacyMonth::January).unwrap();
    if let Some(discriminant) = bytes.as_mut_slice().last_mut() {
      *discriminant = u8::MAX;
    }
    assert!(matches!(
      migrate::<chrono::Month>(&bytes, SourceHint::Auto),
      Err(crate::MigrationError::UnreadableInput { .. })
    ));
  }

  #[test]
  fn converts_iso_week_boundary_values() {
    let min = chrono::NaiveDate::MIN.iso_week();
    let min_packed = (min.year() << 10) | (i32::try_from(min.week()).unwrap() << 4) | i32::from(expected_year_flags(min.year()));
    assert_eq!(
      convert_iso_week(LegacyIsoWeek {
        ywf: min_packed
      })
      .unwrap(),
      min
    );

    let max = chrono::NaiveDate::MAX.iso_week();
    let max_packed = (max.year() << 10) | (i32::try_from(max.week()).unwrap() << 4) | i32::from(expected_year_flags(max.year()));
    assert_eq!(
      convert_iso_week(LegacyIsoWeek {
        ywf: max_packed
      })
      .unwrap(),
      max
    );
  }

  #[test]
  fn migrates_every_supported_current_root_type() {
    let date = LegacyNaiveDate {
      yof: NonZeroI32::new((2024 << 13) | (60 << 4) | i32::from(expected_year_flags(2024))).unwrap(),
    };
    let time = LegacyNaiveTime {
      secs: 45_296,
      frac: 789_000_000,
    };
    let expected_date = chrono::NaiveDate::from_yo_opt(2024, 60).unwrap();
    let expected_time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(45_296, 789_000_000).unwrap();
    let expected_naive = NaiveDateTime::new(expected_date, expected_time);
    let expected_week = expected_date.iso_week();
    let iso_week = LegacyIsoWeek {
      ywf: (expected_week.year() << 10)
        | (i32::try_from(expected_week.week()).unwrap() << 4)
        | i32::from(expected_year_flags(expected_week.year())),
    };
    let fixed = LegacyFixedOffset {
      local_minus_utc: 19_800
    };
    let expected_fixed = FixedOffset::east_opt(19_800).unwrap();

    assert_migrates!(
      LegacyTimeDelta {
        secs:  -2,
        nanos: 750_000_000,
      },
      chrono::TimeDelta,
      chrono::TimeDelta::new(-2, 750_000_000).unwrap()
    );
    assert_migrates!(date, chrono::NaiveDate, expected_date);
    assert_migrates!(time, chrono::NaiveTime, expected_time);
    assert_migrates!(
      LegacyNaiveDateTime {
        date,
        time
      },
      chrono::NaiveDateTime,
      expected_naive
    );
    assert_migrates!(iso_week, chrono::IsoWeek, expected_week);
    assert_migrates!(LegacyUtc, Utc, Utc);
    assert_migrates!(LegacyLocal, Local, Local);
    assert_migrates!(fixed, FixedOffset, expected_fixed);
    assert_migrates!(LegacyMonth::February, chrono::Month, chrono::Month::February);
    assert_migrates!(LegacyWeekday::Thu, chrono::Weekday, chrono::Weekday::Thu);

    assert_migrates!(
      LegacyDateTimeUtc {
        datetime: LegacyNaiveDateTime {
          date,
          time
        },
        offset:   LegacyUtc,
      },
      DateTime<Utc>,
      DateTime::from_naive_utc_and_offset(expected_naive, Utc)
    );
    assert_migrates!(
      LegacyDateTimeFixed {
        datetime: LegacyNaiveDateTime {
          date,
          time
        },
        offset:   fixed,
      },
      DateTime<FixedOffset>,
      DateTime::from_naive_utc_and_offset(expected_naive, expected_fixed)
    );
    assert_migrates!(
      LegacyDateTimeFixed {
        datetime: LegacyNaiveDateTime {
          date,
          time
        },
        offset:   fixed,
      },
      DateTime<Local>,
      DateTime::from_naive_utc_and_offset(expected_naive, expected_fixed)
    );

    assert_migrates!(
      LegacyDateUtc {
        date,
        offset: LegacyUtc,
      },
      Date<Utc>,
      Date::from_utc(expected_date, Utc)
    );
    assert_migrates!(
      LegacyDateFixed {
        date,
        offset: fixed,
      },
      Date<FixedOffset>,
      Date::from_utc(expected_date, expected_fixed)
    );
    assert_migrates!(
      LegacyDateFixed {
        date,
        offset: fixed,
      },
      Date<Local>,
      Date::from_utc(expected_date, expected_fixed)
    );
  }
}
