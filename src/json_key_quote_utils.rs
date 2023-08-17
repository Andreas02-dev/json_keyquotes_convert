//! Core functions used for the JSON conversions.
//!
//! Contains the core functionality of this crate.

use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{load_write_utils, Quotes};

const SUPPORTED_KEY_CHARS_REGEX_STR: &str = r#"A-Za-z0-9`~!@#$%€^&*()\-_=+\\|;"'.<>/?\s"#;

/// Convenience method for chained [load_write_utils::load_json],
/// [json_remove_key_quotes], [json_unescape_ctrlchars]
///  and [load_write_utils::write_json] function calls.
///
/// # Arguments
///
/// * `path` - The file path.
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::Path;
/// use json_keyquotes_convert::{json_key_quote_utils};
///
/// let path = Path::new("./test_resources/Test_with_keyquotes.json");
/// json_key_quote_utils::json_convert_with_to_without_keyquotes(path);
/// ```
pub fn json_convert_with_to_without_keyquotes(path: &Path) {
    let json = match load_write_utils::load_json(path) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let unquoted_json = json_remove_key_quotes(&json);

    match load_write_utils::write_json(path, &json_unescape_ctrlchars(&unquoted_json)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

/// Convenience method for chained [load_write_utils::load_json], [json_add_key_quotes]
/// ,[json_escape_ctrlchars] and [load_write_utils::write_json] calls.
///
/// # Arguments
///
/// * `path` - The file path.
/// * `quote_type` - Whether the JSON keys should be single- or double-quoted.
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::Path;
/// use json_keyquotes_convert::{json_keyquote_utils, Quotes};
///
/// let path = Path::new("./test_resources/Test_without_keyquotes.json")
/// json_keyquote_utils::json_convert_without_to_with_keyquotes(path, Quotes::default());
/// ```
pub fn json_convert_without_to_with_keyquotes(path: &Path, quote_type: Quotes) {
    let json = match load_write_utils::load_json(path) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let keyquoted_json = json_add_key_quotes(&json, quote_type);

    match load_write_utils::write_json(path, &json_escape_ctrlchars(&keyquoted_json)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

/// Adds key-quotes to the JSON string.
///
/// # Arguments
///
/// * `json` - The JSON string.
/// * `quote_type` - Whether the JSON keys should be single- or double-quoted.
///
/// # Examples
///
/// ```
/// use json_keyquotes_convert::{json_key_quote_utils, Quotes};
///
/// let json_added = json_key_quote_utils::json_add_key_quotes("{key: \"val\"}", Quotes::default());
/// assert_eq!(json_added, "{\"key\": \"val\"}");
///
/// let json_already_existing = json_key_quote_utils::json_add_key_quotes("{\"key\": \"val\"}", Quotes::default());
/// assert_eq!(json_already_existing, "{\"key\": \"val\"}");
/// ```
pub fn json_add_key_quotes(json: &str, quote_type: Quotes) -> String {
    // Add quotes around all string keys (single-quoted):
    // `/` == `\/` in Regex101
    let single_quoted_string_val_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<prevchar_key>[^"'][\s]*)(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?[^"'])(?P<val>:\s*?'[\s\S]*?')"#),
        )
        .unwrap()
    });
    let json_single_quoted_string_passed = single_quoted_string_val_regex.replace_all(
        json,
        "$prevchar_key".to_string() + quote_type.as_str() + "$key" + quote_type.as_str() + "$val",
    );

    // Add quotes around all string keys (double-quoted):
    // `/` == `\/` in Regex101
    let double_quoted_string_val_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<prevchar_key>[^"'][\s]*)(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?[^"'])(?P<val>:\s*?"[\s\S]*?")"#),
        )
        .unwrap()
    });
    let json_double_quoted_string_passed = double_quoted_string_val_regex.replace_all(
        &json_single_quoted_string_passed,
        "$prevchar_key".to_string() + quote_type.as_str() + "$key" + quote_type.as_str() + "$val",
    );

    // Add quotes around all object keys:
    // `/` == `\/` in Regex101
    let object_val_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?[^"'])(?P<val>:\s*?[{\[])"#),
        )
        .unwrap()
    });
    let json_object_passed = object_val_regex.replace_all(
        &json_double_quoted_string_passed,
        quote_type.as_str().to_string() + "$key" + quote_type.as_str() + "$val",
    );

    // Add quotes around all number keys:
    // `/` == `\/` in Regex101
    let number_val_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<before>[\[,{]\s*?)(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?[^"'])(?P<after>:\s*?[\d\-\.])"#),
        )
        .unwrap()
    });
    let json_number_passed = number_val_regex.replace_all(
        &json_object_passed,
        "$before".to_string() + quote_type.as_str() + "$key" + quote_type.as_str() + "$after",
    );

    // Add quotes around all `null`, and `boolean` keys:
    // `/` == `\/` in Regex101
    let null_bools_val_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<before>[\[,{]\s*?)(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?[^"'])(?P<after>:\s*?(?:null|true|false))"#),
        )
        .unwrap()
    });
    let json_null_bools_passed = null_bools_val_regex.replace_all(
        &json_number_passed,
        "$before".to_string() + quote_type.as_str() + "$key" + quote_type.as_str() + "$after",
    );

    return json_null_bools_passed.to_string();
}

/// Removes key-quotes from the JSON string.
///
/// # Arguments
///
/// * `json` - The JSON string.
///
/// # Examples
///
/// ```
/// use json_keyquotes_convert::{json_key_quote_utils, Quotes};
///
/// let json_removed = json_key_quote_utils::json_remove_key_quotes("{\"key\": \"val\"}");
/// assert_eq!(json_removed, "{key: \"val\"}");
///
/// let json_already_removed = json_key_quote_utils::json_remove_key_quotes("{key: \"val\"}");
/// assert_eq!(json_already_removed, "{key: \"val\"}");
/// ```
pub fn json_remove_key_quotes(json: &str) -> String {
    // Remove the quotes from the keys (single-quoted):
    // `/` == `\/` in Regex101
    let single_quotes_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<before>[{\[,][\s]*)'(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?)'(?P<after>\s*?:)"#),
        )
        .unwrap()
    });
    let json_single_quotes_passed = single_quotes_regex.replace_all(json, "$before$key$after");

    // Remove the quotes from the keys (double-quoted):
    // `/` == `\/` in Regex101
    let double_quotes_regex = Lazy::new(|| {
        Regex::new(
            &(r#"(?P<before>[{\[,][\s]*)"(?P<key>["#.to_string()
                + SUPPORTED_KEY_CHARS_REGEX_STR
                + r#"]*?)"(?P<after>\s*?:)"#),
        )
        .unwrap()
    });
    let json_double_quotes_passed =
        double_quotes_regex.replace_all(&json_single_quotes_passed, "$before$key$after");

    return json_double_quotes_passed.to_string();
}

/// Escape ctrl-characters from the JSON string values
/// and remove ctrl-characters from the JSON keys with keyquotes.
///
/// This method will escape `newlines`, `tabs` and `carriage returns` in the JSON string values
/// and remove `newlines`, `tabs` and `carriage returns` in the JSON keys with keyquotes.
///
/// # Arguments
///
/// * `json` - The JSON string.
///
/// # Examples
///
/// ```
/// use json_keyquotes_convert::{json_key_quote_utils};
///
/// let json_escaped = json_key_quote_utils::json_escape_ctrlchars(r#"{"key": "va
/// l"}"#);
/// assert_eq!(json_escaped, r#"{"key": "va\nl"}"#);
///
/// let json_already_escaped = json_key_quote_utils::json_escape_ctrlchars(r#"{"key": "va\nl"}"#);
/// assert_eq!(json_already_escaped, r#"{"key": "va\nl"}"#);
/// ```
pub fn json_escape_ctrlchars(json: &str) -> String {
    // Replace all control characters with their escaped variants:

    let mut new_json = json.to_owned();

    // Two iterations are needed for the tab escaping:

    for _n in 0..2 {
        // For all single-quoted string keys with single-quoted values:
        let singlequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)'(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])'(?P<val>\s*?:\s*?'[\s\S]*?')"#),
            )
            .unwrap()
        });
        for cap in singlequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all double-quoted string keys with single-quoted values:
        let singlequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)"(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])"(?P<val>\s*?:\s*?'[\s\S]*?')"#),
            )
            .unwrap()
        });
        for cap in singlequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all single-quoted string keys with double-quoted values:
        let doublequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)'(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])'(?P<val>\s*?:\s*?"[\s\S]*?")"#),
            )
            .unwrap()
        });
        for cap in doublequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all double-quoted string keys with double-quoted values:
        let doublequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)"(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])"(?P<val>\s*?:\s*?"[\s\S]*?")"#),
            )
            .unwrap()
        });
        for cap in doublequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all single-quoted object keys:
        let object_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"'(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])'(?P<val>\s*?:\s*?[{\[])"#),
            )
            .unwrap()
        });
        for cap in object_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all double-quoted object keys:
        let object_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#""(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])"(?P<val>\s*?:\s*?[{\[])"#),
            )
            .unwrap()
        });
        for cap in object_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all single-quoted number keys:
        let number_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)'(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])'(?P<after>\s*?:\s*?[\d\-\.])"#),
            )
            .unwrap()
        });
        for cap in number_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all double-quoted number keys:
        let number_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)"(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])"(?P<after>\s*?:\s*?[\d\-\.])"#),
            )
            .unwrap()
        });
        for cap in number_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all single-quoted null and boolean keys:
        let null_boolean_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)'(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])'(?P<after>\s*?:\s*?(?:null|true|false))"#),
            )
            .unwrap()
        });
        for cap in null_boolean_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all double-quoted null and boolean keys:
        let null_boolean_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)"(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])"(?P<after>\s*?:\s*?(?:null|true|false))"#),
            )
            .unwrap()
        });
        for cap in null_boolean_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\r", ""), 1);
            new_json =
                new_json.replacen(cap_match, &cap_match.replace("\t", "").replace("\t", ""), 1);
        }

        // For all single-quoted string values:
        let singlequoted_string_value_regex =
            Lazy::new(|| Regex::new(r#":[\s]*?'((?:[^'\\]|\\.)*)'"#).unwrap());
        for cap in singlequoted_string_value_regex.captures_iter(&new_json.clone()) {
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\r", "\\r"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\n", "\\n"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\t", "\\t"), 1);
        }

        // For all double-quoted string values:
        let doublequoted_string_value_regex =
            Lazy::new(|| Regex::new(r#":[\s]*?"((?:[^"\\]|\\.)*)""#).unwrap());
        for cap in doublequoted_string_value_regex.captures_iter(&new_json.clone()) {
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\r", "\\r"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\n", "\\n"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\t", "\\t"), 1);
        }
    }

    new_json
}

/// Unescape ctrl-characters from the JSON string values
/// and remove ctrl-characters from the JSON keys without keyquotes.
///
/// This method will unescape `newlines`, `tabs` and `carriage returns` in the JSON string values
/// and remove `newlines`, `tabs` and `carriage returns` in the JSON keys without keyquotes.
///
/// # Arguments
///
/// * `json` - The JSON string.
///
/// # Examples
///
/// ```
/// use json_keyquotes_convert::{json_key_quote_utils};
///
/// let json_unescaped = json_key_quote_utils::json_unescape_ctrlchars(r#"{key: "va\nl"}"#);
/// assert_eq!(json_unescaped, r#"{key: "va
/// l"}"#);
///
/// let json_already_unescaped = json_key_quote_utils::json_unescape_ctrlchars(&json_unescaped);
/// assert_eq!(json_already_unescaped, r#"{key: "va
/// l"}"#);
/// ```
pub fn json_unescape_ctrlchars(json: &str) -> String {
    // Replace all escaped control characters with their unescaped variants:

    let mut new_json = json.to_owned();

    // Two iterations are needed for the tab unescaping:

    for _n in 0..2 {
        // For all single-quoted string keys:
        let singlequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])(?P<val>\s*?:\s*?'[\s\S]*?')"#),
            )
            .unwrap()
        });
        for cap in singlequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\r", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\t", ""), 1);
        }

        // For all double-quoted string keys:
        let doublequoted_string_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<prevchar_key>[^"'][\s]*)(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])(?P<val>\s*?:\s*?"[\s\S]*?")"#),
            )
            .unwrap()
        });
        for cap in doublequoted_string_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\r", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\t", ""), 1);
        }

        // For all object keys:
        let object_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])(?P<val>\s*?:\s*?[{\[])"#),
            )
            .unwrap()
        });
        for cap in object_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\r", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\t", ""), 1);
        }

        // For all number keys:
        let number_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])(?P<after>\s*?:\s*?[\d\-\.])"#),
            )
            .unwrap()
        });
        for cap in number_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\r", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\t", ""), 1);
        }

        // For all null and boolean keys:
        let null_boolean_key_regex = Lazy::new(|| {
            Regex::new(
                &(r#"(?P<before>[\[,{]\s*?)(?P<key>["#.to_string()
                    + SUPPORTED_KEY_CHARS_REGEX_STR
                    + r#"]*?[^"'])(?P<after>\s*?:\s*?(?:null|true|false))"#),
            )
            .unwrap()
        });
        for cap in null_boolean_key_regex.captures_iter(&new_json.clone()) {
            let cap_match = cap.name("key").unwrap().as_str();
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\r", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\n", ""), 1);
            new_json = new_json.replacen(cap_match, &cap_match.replace("\\t", ""), 1);
        }

        // For all single-quoted string values:
        let singlequoted_string_value_regex =
            Lazy::new(|| Regex::new(r#":[\s]*?'((?:[^'\\]|\\.)*)'"#).unwrap());
        for cap in singlequoted_string_value_regex.captures_iter(&new_json.clone()) {
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\r", "\r"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\n", "\n"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\t", "\t"), 1);
        }

        // For all double-quoted string values:
        let doublequoted_string_value_regex =
            Lazy::new(|| Regex::new(r#":[\s]*?"((?:[^"\\]|\\.)*)""#).unwrap());
        for cap in doublequoted_string_value_regex.captures_iter(&new_json.clone()) {
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\r", "\r"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\n", "\n"), 1);
            new_json = new_json.replacen(&cap[1], &cap[1].replace("\\t", "\t"), 1);
        }
    }

    new_json
}

#[cfg(test)]
mod tests {
    use crate::{json_key_quote_utils, load_write_utils, Quotes};
    use std::path::Path;

    const SUPPORTED_KEY_CHARS: &str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%€^&*()-_=+\|;"'.<>/?"#;
    const SUPPORTED_VALUE_CHARS: &str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%€^&*()-_=+\|:;"'.<>/?"#;

    #[test]
    fn test_json_convert_without_to_with_keyquotes() {
        let path = Path::new("./tmp_without_keyquotes");
        std::fs::copy(
            "./test_resources/Test_without_keyquotes.json",
            "./tmp_without_keyquotes",
        )
        .unwrap();
        json_key_quote_utils::json_convert_without_to_with_keyquotes(
            path,
            crate::Quotes::DoubleQuote,
        );
        let converted_file_contents = load_write_utils::load_json(path).unwrap();
        let expected_file_contents =
            load_write_utils::load_json(Path::new("./test_resources/Test_with_keyquotes.json"))
                .unwrap();
        assert!(converted_file_contents == expected_file_contents);
        std::fs::remove_file("./tmp_without_keyquotes").unwrap();
    }

    #[test]
    fn test_json_convert_with_to_without_keyquotes() {
        let path = Path::new("./tmp_with_keyquotes");
        std::fs::copy(
            "./test_resources/Test_with_keyquotes.json",
            "./tmp_with_keyquotes",
        )
        .unwrap();
        json_key_quote_utils::json_convert_with_to_without_keyquotes(path);
        let converted_file_contents = load_write_utils::load_json(path).unwrap();
        let expected_file_contents =
            load_write_utils::load_json(Path::new("./test_resources/Test_without_keyquotes.json"))
                .unwrap();
        assert!(converted_file_contents == expected_file_contents);
        std::fs::remove_file("./tmp_with_keyquotes").unwrap();
    }

    #[test]
    fn test_json_add_key_quotes_single_quote_add_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"'"#, r#"\'"#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"'"#, r#"\'"#, 1);

        let json =
            r#"{"#.to_string() + &supported_key_chars + r#": '"# + &supported_value_chars + r#"'}"#;
        let expected = r#"{'"#.to_string()
            + &supported_key_chars
            + r#"': '"#
            + &supported_value_chars
            + r#"'}"#;

        let actual = json_key_quote_utils::json_add_key_quotes(&json, Quotes::SingleQuote);
        let actual_second_pass =
            json_key_quote_utils::json_add_key_quotes(&actual, Quotes::SingleQuote);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_add_key_quotes_double_quote_add_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"""#, r#"\""#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let json =
            r#"{"#.to_string() + &supported_key_chars + r#": ""# + &supported_value_chars + r#""}"#;
        let expected = r#"{""#.to_string()
            + &supported_key_chars
            + r#"": ""#
            + &supported_value_chars
            + r#""}"#;

        let actual = json_key_quote_utils::json_add_key_quotes(&json, Quotes::DoubleQuote);
        let actual_second_pass =
            json_key_quote_utils::json_add_key_quotes(&actual, Quotes::DoubleQuote);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_remove_key_quotes_single_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"'"#, r#"\'"#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"'"#, r#"\'"#, 1);

        let json = r#"{'"#.to_string()
            + &supported_key_chars
            + r#"': ""#
            + &supported_value_chars
            + r#""}"#;
        let expected =
            r#"{"#.to_string() + &supported_key_chars + r#": ""# + &supported_value_chars + r#""}"#;

        let actual = json_key_quote_utils::json_remove_key_quotes(&json);
        let actual_second_pass = json_key_quote_utils::json_remove_key_quotes(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_remove_key_quotes_double_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"""#, r#"\""#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let json = r#"{""#.to_string()
            + &supported_key_chars
            + r#"": ""#
            + &supported_value_chars
            + r#""}"#;
        let expected =
            r#"{"#.to_string() + &supported_key_chars + r#": ""# + &supported_value_chars + r#""}"#;

        let actual = json_key_quote_utils::json_remove_key_quotes(&json);
        let actual_second_pass = json_key_quote_utils::json_remove_key_quotes(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_escape_ctrlchars_single_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"'"#, r#"\'"#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"'"#, r#"\'"#, 1);

        let key = supported_key_chars.to_string();
        let value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let expected_key = supported_key_chars.to_string();
        let expected_value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let json = r#"{'"#.to_string() + &key + r#"': '"# + &value + r#"'}"#;
        let expected = r#"{'"#.to_string() + &expected_key + r#"': '"# + &expected_value + r#"'}"#;

        let actual = json_key_quote_utils::json_escape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_escape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_escape_ctrlchars_double_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"""#, r#"\""#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let key = supported_key_chars.to_string();
        let value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let expected_key = supported_key_chars.to_string();
        let expected_value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let json = r#"{""#.to_string() + &key + r#"": ""# + &value + r#""}"#;
        let expected = r#"{""#.to_string() + &expected_key + r#"": ""# + &expected_value + r#""}"#;

        let actual = json_key_quote_utils::json_escape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_escape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_escape_ctrlchars_unquoted_keys_supported_characters() {
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let key = SUPPORTED_KEY_CHARS
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);
        let value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let expected_value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let json = r#"{"#.to_string() + &key + r#": ""# + &value + r#""}"#;
        let expected = r#"{"#.to_string() + &key + r#": ""# + &expected_value + r#""}"#;

        let actual = json_key_quote_utils::json_escape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_escape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_unescape_ctrlchars_single_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"'"#, r#"\'"#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"'"#, r#"\'"#, 1);

        let key = supported_key_chars.to_string();
        let value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let expected_key = supported_key_chars.to_string();
        let expected_value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let json = r#"{"#.to_string() + &key + r#": '"# + &value + r#"'}"#;
        let expected = r#"{"#.to_string() + &expected_key + r#": '"# + &expected_value + r#"'}"#;

        let actual = json_key_quote_utils::json_unescape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_unescape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_unescape_ctrlchars_double_quoted_supported_characters() {
        let supported_key_chars = SUPPORTED_KEY_CHARS.replacen(r#"""#, r#"\""#, 1);
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let key = supported_key_chars.to_string();
        let value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let expected_key = supported_key_chars.to_string();
        let expected_value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let json = r#"{"#.to_string() + &key + r#": ""# + &value + r#""}"#;
        let expected = r#"{"#.to_string() + &expected_key + r#": ""# + &expected_value + r#""}"#;

        let actual = json_key_quote_utils::json_unescape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_unescape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }

    #[test]
    fn test_json_unescape_ctrlchars_double_quoted_keys_supported_characters() {
        let supported_value_chars = SUPPORTED_VALUE_CHARS.replacen(r#"""#, r#"\""#, 1);

        let key = SUPPORTED_KEY_CHARS
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);
        let value = supported_value_chars
            .replacen("A", r#"A\n"#, 1)
            .replacen("B", r#"B\t"#, 1);

        let expected_value = supported_value_chars
            .replacen(
                "A", r#"A
"#, 1,
            )
            .replacen("B", r#"B	"#, 1);

        let json = r#"{""#.to_string() + &key + r#"": ""# + &value + r#""}"#;
        let expected = r#"{""#.to_string() + &key + r#"": ""# + &expected_value + r#""}"#;

        let actual = json_key_quote_utils::json_unescape_ctrlchars(&json);
        let actual_second_pass = json_key_quote_utils::json_unescape_ctrlchars(&actual);

        assert_eq!(expected, actual);
        assert_eq!(expected, actual_second_pass);
    }
}
