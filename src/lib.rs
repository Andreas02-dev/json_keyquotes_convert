use std::io;
use std::path::Path;
use std::fs;
use regex::Regex;

pub fn json_convert_with_to_without_keyquotes(path: &Path) {
        
    let json = match load_json(path) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let unquoted_json = json_remove_key_quotes(&json);

    match write_json(path, &json_unescape_ctrlchars(&unquoted_json)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

pub fn json_convert_without_to_with_keyquotes(path: &Path) {
        
    let json = match load_json(path) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let escaped_json = json_escape_ctrlchars(&json);

    match write_json(path, &json_add_key_quotes(&escaped_json)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

fn load_json(path: &Path) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
        Ok(val) => return Ok(val),
        Err(err) => return Err(err)
    };
}

fn write_json(path: &Path, json: &str) -> Result<(), io::Error> {
    match fs::write(path, json) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err)
    }
}

fn json_add_key_quotes(json: &str) -> String {
    
    // Add quotes around all string keys:
    let string_val_regex = Regex::new(r#"(?P<key>[A-Za-z0-9]*?)(?P<val>:\s*?"[\s\S]*?")"#).unwrap();
    let json_string_passed = string_val_regex.replace_all(json, "\"$key\"$val");

    // Add quotes around all object keys:
    let object_val_regex = Regex::new(r"(?P<key>[A-Za-z0-9]*?)(?P<val>:\s*?[{\[])").unwrap();
    let json_object_passed = object_val_regex.replace_all(&json_string_passed, "\"$key\"$val");

    // Add quotes around all number keys:
    let number_val_regex = Regex::new(r"(?P<before>[\[,{]\s*?)(?P<key>[A-Za-z0-9]*?)(?P<after>:\s*?[\d\-\.])").unwrap();
    let json_number_passed = number_val_regex.replace_all(&json_object_passed, "$before\"$key\"$after");

    // Add quotes around all null keys:
    let null_val_regex = Regex::new(r"(?P<before>[\[,{]\s*?)(?P<key>[A-Za-z0-9]*?)(?P<after>:\s*?null)").unwrap();
    let json_null_passed = null_val_regex.replace_all(&json_number_passed, "$before\"$key\"$after");

    // Add quotes around all boolean-true keys:
    let true_val_regex = Regex::new(r"(?P<before>[\[,{]\s*?)(?P<key>[A-Za-z0-9]*?)(?P<after>:\s*?true)").unwrap();
    let json_true_passed = true_val_regex.replace_all(&json_null_passed, "$before\"$key\"$after");

    // Add quotes around all boolean-false keys:
    let false_val_regex = Regex::new(r"(?P<before>[\[,{]\s*?)(?P<key>[A-Za-z0-9]*?)(?P<after>:\s*?false)").unwrap();
    let json_false_passed = false_val_regex.replace_all(&json_true_passed, "$before\"$key\"$after");

    return json_false_passed.to_string();
}

fn json_remove_key_quotes(json: &str) -> String {

    // Remove the quotes from the keys:
    let quotes_regex = Regex::new(r#"(?P<before>[{\[,][\s]*)"(?P<key>[A-Za-z0-9]*?)"(?P<after>:)"#).unwrap();
    let json_quotes_passed = quotes_regex.replace_all(json, "$before$key$after");

    return json_quotes_passed.to_string();
}

fn json_escape_ctrlchars(json: &str) -> String {

    // Replace all control characters with their escaped variants:

    let mut new_json = json.to_owned();

    let string_regex = Regex::new(r#""((?:[^"\\]|\\.)*)""#).unwrap();
    for cap in string_regex.captures_iter(json) {
        new_json = new_json.replacen(&cap[1], &cap[1].replace("\n", "\\n"), 1);
        new_json = new_json.replacen(&cap[1], &cap[1].replace("\t", "\\t"), 1);
    }
    return new_json;
}

fn json_unescape_ctrlchars(json: &str) -> String {

    // Replace all escaped control characters with their unescaped variants:

    let mut new_json = json.to_owned();

    let string_regex = Regex::new(r#""((?:[^"\\]|\\.)*)""#).unwrap();
    for cap in string_regex.captures_iter(json) {
        new_json = new_json.replacen(&cap[1], &cap[1].replace("\\n", "\n"), 1);
        new_json = new_json.replacen(&cap[1], &cap[1].replace("\\t", "\t"), 1);
    }
    return new_json;
}


#[cfg(test)]
mod tests {
    use std::{path::Path};
    use crate::{json_convert_without_to_with_keyquotes, load_json, json_convert_with_to_without_keyquotes};

    #[test]
    fn test_json_convert_without_to_with_keyquotes() {
        let path = Path::new("./tmp_without_keyquotes");
        std::fs::copy("./test_resources/Test_without_keyquotes.json", "./tmp_without_keyquotes").unwrap();
        json_convert_without_to_with_keyquotes(path);
        let converted_file_contents = load_json(path).unwrap();
        let expected_file_contents = load_json(Path::new("./test_resources/Test_with_keyquotes.json")).unwrap();
        assert!(converted_file_contents == expected_file_contents);
        std::fs::remove_file("./tmp_without_keyquotes").unwrap();
    }

    #[test]
    fn test_json_convert_with_to_without_keyquotes() {
        let path = Path::new("./tmp_with_keyquotes");
        std::fs::copy("./test_resources/Test_with_keyquotes.json", "./tmp_with_keyquotes").unwrap();
        json_convert_with_to_without_keyquotes(path);
        let converted_file_contents = load_json(path).unwrap();
        let expected_file_contents = load_json(Path::new("./test_resources/Test_without_keyquotes.json")).unwrap();
        assert!(converted_file_contents == expected_file_contents);
        std::fs::remove_file("./tmp_with_keyquotes").unwrap();
    }
}
