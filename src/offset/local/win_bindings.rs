windows_link::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : PDYNAMIC_TIME_ZONE_INFORMATION, ptzi : LPTIME_ZONE_INFORMATION) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const SYSTEMTIME, lpfiletime : LPFILETIME) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const SYSTEMTIME, lplocaltime : LPSYSTEMTIME) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const SYSTEMTIME, lpuniversaltime : LPSYSTEMTIME) -> BOOL);
pub type BOOL = i32;
pub type BOOLEAN = u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
  pub Bias: i32,
  pub StandardName: [u16; 32],
  pub StandardDate: SYSTEMTIME,
  pub StandardBias: i32,
  pub DaylightName: [u16; 32],
  pub DaylightDate: SYSTEMTIME,
  pub DaylightBias: i32,
  pub TimeZoneKeyName: [u16; 128],
  pub DynamicDaylightTimeDisabled: BOOLEAN,
}
impl Default for DYNAMIC_TIME_ZONE_INFORMATION {
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
  pub dwLowDateTime:  u32,
  pub dwHighDateTime: u32,
}
pub type LPFILETIME = *mut FILETIME;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEMTIME {
  pub wYear:         u16,
  pub wMonth:        u16,
  pub wDayOfWeek:    u16,
  pub wDay:          u16,
  pub wHour:         u16,
  pub wMinute:       u16,
  pub wSecond:       u16,
  pub wMilliseconds: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TIME_ZONE_INFORMATION {
  pub Bias:         i32,
  pub StandardName: [u16; 32],
  pub StandardDate: SYSTEMTIME,
  pub StandardBias: i32,
  pub DaylightName: [u16; 32],
  pub DaylightDate: SYSTEMTIME,
  pub DaylightBias: i32,
}
impl Default for TIME_ZONE_INFORMATION {
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}
