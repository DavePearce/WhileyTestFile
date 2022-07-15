use whiley_test_file::{Error,WhileyTestFile,Value};

// ===============================================================
// Config Tests
// ===============================================================

#[test]
fn config_int_01() {
    parse_config_option("hello = 1","hello",Value::Int(1));
}

#[test]
fn config_int_02() {
    parse_config_option("h = 1234565","h",Value::Int(1234565));
}

#[test]
fn config_int_03() {
    parse_config_option("h = -1","h",Value::Int(-1));
}

#[test]
fn config_bool_01() {
    parse_config_option("hello = false","hello",Value::Bool(false));
}

#[test]
fn config_bool_02() {
    parse_config_option("hello = true","hello",Value::Bool(true));
}

#[test]
fn config_string_01() {
    parse_config_option("s = \"world\"","s",Value::String("world".to_string()));
}

#[test]
fn config_invalid_01() {
    let wtf = parse_expecting("hello ", Error::InvalidConfigOption);
}

#[test]
fn config_invalid_02() {
    let wtf = parse_expecting("hello = ", Error::InvalidConfigValue);
}

#[test]
fn config_invalid_03() {
    parse_expecting("hello = t",Error::InvalidConfigValue);
}

#[test]
fn config_int_invalid_01() {
    parse_expecting("hello = 1c",Error::InvalidIntValue);
}

#[test]
fn config_int_invalid_02() {
    parse_expecting("hello = -1c",Error::InvalidIntValue);
}

#[test]
fn config_string_invalid_01() {
    parse_expecting("hello = \"",Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_02() {
    parse_expecting("hello = \"x",Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_03() {
    parse_expecting("hello = \"\"\"",Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_04() {
    parse_expecting("hello = \"x\"x\"",Error::InvalidStringValue);
}

// ===============================================================
// Single Frame Tests
// ===============================================================

#[test]
fn single_frame_01() {
    let wtf = parse(r#"
====
>>> main.whiley
type nat is (int x)"#
    );
    //
    assert!(wtf.size() == 1);
}


// ===============================================================
// Helpers
// ===============================================================

fn parse(input: &str) -> WhileyTestFile {
    // Parser test file
    let wtf = WhileyTestFile::from_str(input);
    // Assume parsing succeeded
    assert!(wtf.is_ok());
    // Done
    wtf.unwrap()
}

fn parse_config_option(input: &str, key: &str, val: Value) {
    // Parser test file
    let wtf = WhileyTestFile::from_str(input).unwrap();
    // Look for value
    match wtf.get(key) {
        Some(v) => {
            if &val != v {
                panic!("Expecting config value {:?} for key {:?}, got {:?}!",val,key,v);
            }
        }
        _ => {
            panic!("Expected config value {:?} for key {:?}, got nothing!",val,key);
        }
    }
}

fn parse_expecting(input: &str, expected: Error) {
    // Parser test file
    match WhileyTestFile::from_str(input) {
        Ok(_) => {
            panic!("File should not have parsed!");
        }
        Err(err) => {
            if err != expected {
                panic!("Expected error {:?}, got {:?}!",expected,err);
            }
        }
    }
}
