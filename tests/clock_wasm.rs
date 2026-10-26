//! Fallible JavaScript clock sampling and conversion, including invalid dates.

#![cfg(all(
  target_family = "wasm",
  feature = "wasmbind",
  not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
))]

use chrono::DateTime;
use chrono::DateTimeConversionError;
use chrono::Utc;
use js_sys::Date;
use num_traits::ToPrimitive;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn preserves_signed_milliseconds() {
  for millis in [-1_000_001, -1_000, -1, 0, 1, 1_000, 1_000_001] {
    let sample = Date::new(&JsValue::from_f64(f64::from(millis)));
    assert_eq!(
      Some(DateTime::<Utc>::from_js_date(sample).unwrap()),
      DateTime::from_timestamp_millis(i64::from(millis)),
    );
  }
}

#[wasm_bindgen_test]
fn retains_invalid_date_and_observed_nan() {
  let sample = Date::new(&JsValue::from_f64(f64::NAN));
  let original = sample.clone();
  let failure = DateTime::<Utc>::from_js_date(sample).unwrap_err();
  match &failure {
    DateTimeConversionError::JsDate {
      sample: observed,
      milliseconds,
    } => {
      assert_eq!(observed, &original);
      assert!(milliseconds.is_nan());
      // Mutating the JavaScript object must not rewrite the recorded observation.
      original.set_time(0.0);
      assert_eq!(original.get_time(), 0.0);
      assert!(milliseconds.is_nan());
    }
    unexpected => panic!("expected invalid date, got {unexpected:?}"),
  }
  // Diagnostics describe the failed observation even after the retained object changes.
  assert_eq!(
    failure.to_string(),
    "JavaScript date with timestamp NaN ms is invalid or outside Chrono's calendar range",
  );
}

#[wasm_bindgen_test]
fn accepts_calendar_boundaries_and_rejects_outside_dates() {
  for endpoint in [DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC] {
    let millis = endpoint.timestamp_millis();
    let sample = Date::new(&JsValue::from_f64(millis.to_f64().unwrap()));
    assert_eq!(
      Some(DateTime::<Utc>::from_js_date(sample).unwrap()),
      DateTime::from_timestamp_millis(millis),
    );
  }
  for milliseconds in [-8_640_000_000_000_000.0, 8_640_000_000_000_000.0] {
    let sample = Date::new(&JsValue::from_f64(milliseconds));
    let original = sample.clone();
    match DateTime::<Utc>::from_js_date(sample) {
      Err(DateTimeConversionError::JsDate {
        sample: observed,
        milliseconds: recorded,
      }) => {
        assert_eq!(observed, original);
        assert_eq!(recorded, milliseconds);
      }
      unexpected => panic!("expected calendar overflow, got {unexpected:?}"),
    }
  }
}

#[cfg(feature = "now")]
#[wasm_bindgen_test]
fn acquires_fresh_javascript_wall_time() {
  let before = DateTime::<Utc>::from_js_date(Date::new_0()).unwrap();
  let sampled = Utc::try_now().unwrap();
  let after = DateTime::<Utc>::from_js_date(Date::new_0()).unwrap();
  assert!((before.min(after)..=before.max(after)).contains(&sampled));
  assert_eq!(DateTime::from_timestamp_millis(sampled.timestamp_millis()), Some(sampled));
}
