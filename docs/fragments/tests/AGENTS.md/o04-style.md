## Style

Library layer — kept mergeable with upstream `chronotope/chrono`. `#[test]`/`#[wasm_bindgen_test]` functions returning `()`, `assert_eq!`/`assert!`, `unwrap()`, and the module-level `#[allow(dead_code)]` on the cfg-conditional `DATE_PATH` are intentional upstream idioms; do not convert them to the strict `Result<(), TestFailure>` + `ensure*` vocabulary.
