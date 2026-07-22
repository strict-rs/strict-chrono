## Range, sentinels, and the out-of-range buffer

`NaiveDate` spans about ±262,143 years. `MIN_YEAR`/`MAX_YEAR` sit one year inside the raw capacity on purpose: the extra headroom holds `NaiveDate::BEFORE_MIN` and `AFTER_MAX`, which `NaiveDateTime::overflowing_add/sub_offset` fall back to. This is the naive-layer half of the `DateTime::overflowing_naive_local` trick — an out-of-range local value can exist transiently but must never be handed to a user.
