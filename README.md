### Status
[![Tests](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/tests.yml)
[![Crates.io](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/publish.yml/badge.svg?branch=main)](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/publish.yml)

### Crate information

* [Crates.io](https://crates.io/crates/json_keyquotes_convert)
* [Docs.rs](https://docs.rs/json_keyquotes_convert/)

### Usage

- `json_convert_with_to_without_keyquotes` converts a file from JSON with keyquotes and (\t and \n) escape characters to JSON without keyquotes and (\t and \n) escape characters.
- `json_convert_without_to_with_keyquotes` converts a file from JSON without keyquotes and (\t and \n) escape characters to JSON with keyquotes and (\t and \n) escape characters.

### Important

Please note that this crate does not check whether the output is valid JSON. The functionality of this crate is based on Regular Expressions and uses the [regex](https://crates.io/crates/regex) crate.

### Third party licenses

-  _**Regex**_: Copyright (c) 2014 The Rust Project Developers.
	Licensed under the MIT license, see [LICENSES.MIT](./external/licenses/LICENSES.MIT) for details.
	The notice can be found at [Regex-NOTICE](./external/notices/Regex-NOTICE).
	Website: <https://github.com/rust-lang/regex>.
