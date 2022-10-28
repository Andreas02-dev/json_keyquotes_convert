//! Documentation for the `json_keyquotes_convert` crate.
//!
//! The `json_keyquotes_convert` crate is meant to be used to perform
//! various transformations to JSON, including but not limited to
//! adding and removing quotes around the JSON keys.
//! 
//! It is recommended to use the [JsonKeyQuoteConverter] builder,
//! but using the core functions in [json_key_quote_utils] is possible too.

pub mod json_key_quote_utils;
pub mod load_write_utils;

/// The quotes to use for the JSON keys.
/// 
/// This does not affect existing single-quoted or double-quoted keys in JSON.
/// 
/// The default value is [Quotes::DoubleQuote].
#[derive(Clone, Copy)]
pub enum Quotes {
    DoubleQuote,
    SingleQuote
}

impl Quotes {
    fn as_str(&self) -> &'static str {
        match self {
            Quotes::DoubleQuote => "\"",
            Quotes::SingleQuote => "'"
        }
    }
}

impl Default for Quotes {
    fn default() -> Self { Quotes::DoubleQuote }
}

/// The builder for the JSON conversions.
pub struct JsonKeyQuoteConverter {
    json: String,
    quote_type: Quotes
}

impl JsonKeyQuoteConverter {

    /// Returns a new [JsonKeyQuoteConverter].
    /// 
    /// # Arguments
    /// 
    /// * `json` - A JSON string.
    /// * `quote_type` - Whether the JSON keys should be single- or double-quoted.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let converter = JsonKeyQuoteConverter::new("{\"key\": \"val\"}", Quotes::default());
    /// ```
    pub fn new(json: &str, quote_type: Quotes) -> JsonKeyQuoteConverter {
        JsonKeyQuoteConverter {
            json: String::from(json),
            quote_type: quote_type,
        }
    }

    /// Adds key-quotes to the JSON string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let json_added = JsonKeyQuoteConverter::new("{key: \"val\"}", Quotes::default())
    ///     .add_key_quotes().json();
    /// assert_eq!(json_added, "{\"key\": \"val\"}");
    /// 
    /// let json_already_existing = JsonKeyQuoteConverter::new("{\"key\": \"val\"}", Quotes::default())
    ///     .add_key_quotes().json();
    /// assert_eq!(json_already_existing, "{\"key\": \"val\"}");
    /// ```
    pub fn add_key_quotes(mut self) -> JsonKeyQuoteConverter {

        self.json = json_key_quote_utils::json_add_key_quotes(&self.json, self.quote_type);
        
        self
    }

    /// Removes key-quotes from the JSON string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let json_removed = JsonKeyQuoteConverter::new("{\"key\": \"val\"}", Quotes::default())
    ///     .remove_key_quotes().json();
    /// assert_eq!(json_removed, "{key: \"val\"}");
    /// 
    /// let json_already_removed = JsonKeyQuoteConverter::new("{key: \"val\"}", Quotes::default())
    ///     .remove_key_quotes().json();
    /// assert_eq!(json_already_removed, "{key: \"val\"}");
    /// ```
    pub fn remove_key_quotes(mut self) -> JsonKeyQuoteConverter {

        self.json = json_key_quote_utils::json_remove_key_quotes(&self.json);

        self
    }

    /// Escape ctrl-characters from the JSON string values
    /// and the JSON keys with keyquotes.
    /// 
    /// This method will escape `newlines` and `tabs` in the JSON string values
    /// and in the JSON keys with keyquotes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let json_escaped = JsonKeyQuoteConverter::new(r#"{"ke
    /// y": "va
    /// l"}"#, Quotes::default())
    ///     .escape_ctrlchars().json();
    /// assert_eq!(json_escaped, r#"{"ke\ny": "va\nl"}"#);
    /// 
    /// let json_already_escaped = JsonKeyQuoteConverter::new(r#"{"ke\ny": "va\nl"}"#, Quotes::default())
    ///     .escape_ctrlchars().escape_ctrlchars().json();
    /// assert_eq!(json_already_escaped, r#"{"ke\ny": "va\nl"}"#);
    /// ```
    pub fn escape_ctrlchars(mut self) -> JsonKeyQuoteConverter {

        self.json = json_key_quote_utils::json_escape_ctrlchars(&self.json);

        self
    }

    /// Unescape ctrl-characters from the JSON string values
    /// and the JSON keys without keyquotes.
    /// 
    /// This method will unescape `newlines` and `tabs` in the JSON string values
    /// and in the JSON keys without keyquotes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let json_unescaped = JsonKeyQuoteConverter::new(r#"{ke\ny: "va\nl"}"#, Quotes::default())
    ///     .unescape_ctrlchars().json();
    /// assert_eq!(json_unescaped, r#"{ke
    /// y: "va
    /// l"}"#);
    /// 
    /// let json_already_unescaped = JsonKeyQuoteConverter::new(r#"{ke\ny: "va\nl"}"#, Quotes::default())
    ///     .unescape_ctrlchars().unescape_ctrlchars().json();
    /// assert_eq!(json_already_unescaped, r#"{ke
    /// y: "va
    /// l"}"#);
    /// ```
    pub fn unescape_ctrlchars(mut self) -> JsonKeyQuoteConverter {

        self.json = json_key_quote_utils::json_unescape_ctrlchars(&self.json);

        self
    }

    /// Returns the JSON string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_keyquotes_convert::{JsonKeyQuoteConverter, Quotes};
    /// 
    /// let json = JsonKeyQuoteConverter::new(r#"{"key": "value"}"#, Quotes::default())
    ///     .json();
    /// assert_eq!(json, r#"{"key": "value"}"#);
    /// ```
    pub fn json(self) -> String {
        self.json
    }
}