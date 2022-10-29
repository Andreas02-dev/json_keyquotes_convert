### Crate status
[![crates.io](https://img.shields.io/crates/v/json_keyquotes_convert.svg)](https://crates.io/crates/json_keyquotes_convert)
[![docs.rs](https://img.shields.io/docsrs/json_keyquotes_convert)](https://docs.rs/json_keyquotes_convert)
[![License](https://img.shields.io/crates/l/json_keyquotes_convert.svg)](LICENSE)
### Pipeline status
[![Test pipeline](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/tests.yml)
[![Crates.io deployment pipeline](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/publish.yml/badge.svg?branch=main)](https://github.com/Andreas02-dev/json_keyquotes_convert_rs/actions/workflows/publish.yml)

### Installation
```
cargo add json_keyquotes_convert
```

### Example usage

- For more information, look at the [docs](https://docs.rs/json_keyquotes_convert).

##### Using the builder pattern (recommended):
```rust
use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};

let json = JsonKeyQuoteConverter::new("{key: \"va\nl\"}", Quotes::default())
	.add_key_quotes().json_escape_ctrlchars().json();

// JSON string will now be: {\"key\": \"va\\nl\"}
// Raw JSON will now be: {"key": "va\nl"}
```

##### Using functions:
```rust
use json_keyquotes_convert::{json_key_quote_utils, Quotes};

let json_added = json_key_quote_utils::json_add_key_quotes("{key: \"va\nl\"}", Quotes::default());
let json_escaped = json_key_quote_utils::json_escape_ctrlchars(&json_added);

// JSON string will now be: {\"key\": \"va\\nl\"}
// Raw JSON will now be: {"key": "va\nl"}
```

### Important information

#### Crate support legend

|       Great        |         Good        |  Unsupported  |
| :----------------: | :-----------------: | :-----------: |
| :heavy_check_mark: |  :white_check_mark: |     :x:       |
|   Automatically    |     Configurable    |  Unsupported  |

#### Crate support
** Any unlisted features might be unsupported. **
  - Adding quotes around JSON keys:
  	- Double-quotes: :heavy_check_mark: (default)
	- Single-quotes: :white_check_mark:
  - Removing quotes around JSON keys:
	- Double-quotes: :heavy_check_mark:
	- Single-quotes: :heavy_check_mark:
  - Supported quotes around JSON string values:
	- Double-quotes: :heavy_check_mark:
	- Single-quotes: :heavy_check_mark:
  - Supports control character escaping in JSON keys:
	- Newline (\n): :heavy_check_mark: :white_check_mark:
	- Tab (\t): :heavy_check_mark: :white_check_mark:
	- Note: These characters could be misinterpreted as not belonging to the key. It is therefore not recommended to start or end a JSON key with these characters.
  - Supports control character unescaping in JSON keys:
	- Newline (\n): :heavy_check_mark: :white_check_mark:
	- Tab (\t): :heavy_check_mark: :white_check_mark:
	- Note: These characters could be misinterpreted as not belonging to the key. It is therefore not recommended to start or end a JSON key with these characters.
  - Supports control character escaping in JSON string values:
	- Newline (\n): :heavy_check_mark: :white_check_mark:
	- Tab (\t): :heavy_check_mark: :white_check_mark:
  - Supports control character unescaping in JSON string values:
	- Newline (\n): :heavy_check_mark: :white_check_mark:
	- Tab (\t): :heavy_check_mark: :white_check_mark:
  - Supported :heavy_check_mark: characters in JSON keys:
	- [A-Z] [a-z] [0-9] \` ~ ! @ # $ % € ^ & * ( ) - _ = + \ | ; " ' . < > / ? \r \n \t \f \v `<U+0020>(Space)`
	- Note: ' and " and their escaped variants could be misinterpreted as keyquotes when used as the last character in a JSON key. It is therefore not recommended to start or end a JSON key with these characters.
  - Supported :heavy_check_mark: characters in JSON values:
    - [A-Z] [a-z] [0-9] \` ~ ! @ # $ % € ^ & * ( ) - _ = + \ | : ; " ' . < > / ? \r \n \t \f \v `<U+0020>(Space)`
	
#### Please note that this crate does not check whether the output is valid JSON. The functionality of this crate is based on Regular Expressions and uses the [regex](https://crates.io/crates/regex) crate.

### Data format

#### Similar data-formats

The JSON data-format as shown in the tests shares some similarities with both JSON5 and HJSON. Unfortunately, these data-formats differ enough that it can not be parsed by the JSON5 and HJSON parsers, which is why this crate was made.

#### Usage

The JSON data-format as shown in the tests is used by the following software:
  - [VideoPsalm](https://myvideopsalm.weebly.com/)

### Changelog

- See the [changelog](./CHANGELOG.md).

### Contributing

- All contributions are welcome. I will do my best to reply to all questions and PR's.
- Please do note that all contributions made to this crate will be made available using the current license (MIT license).

### Third party licenses

-  _**Regex**_: Copyright (c) 2014 The Rust Project Developers.
	Licensed under the MIT license, see [LICENSES.MIT](./external/licenses/LICENSES.MIT) for details.
	The notice can be found at [Regex-NOTICE](./external/notices/Regex-NOTICE).
	Website: <https://github.com/rust-lang/regex>.

-  _**once_cell**_: Licensed under the MIT license, see [LICENSES.MIT](./external/licenses/LICENSES.MIT) for details.
	Website: <https://github.com/matklad/once_cell>.