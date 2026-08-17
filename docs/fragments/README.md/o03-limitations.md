## Limitations

* Only the proleptic Gregorian calendar (i.e. extended to support older dates) is supported.
* Date types are limited to about +/- 262,000 years from the common epoch.
* Time types are limited to nanosecond accuracy.
* Leap seconds can be represented, but Chrono does not fully support them.
  See [Leap Second Handling](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html#leap-second-handling).
