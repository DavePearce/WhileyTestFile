use whiley_test_file::{Action, Coordinate, Error, Range, Value, WhileyTestFile};

// ===============================================================
// Config Tests
// ===============================================================

#[test]
fn config_int_01() {
    parse_config_option("hello = 1", "hello", Value::Int(1));
}

#[test]
fn config_int_02() {
    parse_config_option("h = 1234565", "h", Value::Int(1234565));
}

#[test]
fn config_int_03() {
    parse_config_option("h = -1", "h", Value::Int(-1));
}

#[test]
fn config_bool_01() {
    parse_config_option("hello = false", "hello", Value::Bool(false));
}

#[test]
fn config_bool_02() {
    parse_config_option("hello = true", "hello", Value::Bool(true));
}

#[test]
fn config_string_01() {
    parse_config_option("s = \"world\"", "s", Value::String("world".to_string()));
}

#[test]
fn config_invalid_01() {
    let _wtf = parse_expecting("hello ", Error::InvalidConfigOption);
}

#[test]
fn config_invalid_02() {
    let _wtf = parse_expecting("hello = ", Error::InvalidConfigValue);
}

#[test]
fn config_invalid_03() {
    parse_expecting("hello = t", Error::InvalidConfigValue);
}

#[test]
fn config_int_invalid_01() {
    parse_expecting("hello = 1c", Error::InvalidIntValue);
}

#[test]
fn config_int_invalid_02() {
    parse_expecting("hello = -1c", Error::InvalidIntValue);
}

#[test]
fn config_string_invalid_01() {
    parse_expecting("hello = \"", Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_02() {
    parse_expecting("hello = \"x", Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_03() {
    parse_expecting("hello = \"\"\"", Error::InvalidStringValue);
}

#[test]
fn config_string_invalid_04() {
    parse_expecting("hello = \"x\"x\"", Error::InvalidStringValue);
}

// ===============================================================
// Single Frame Tests
// ===============================================================

#[test]
fn single_frame_01() {
    // Frame with insert action
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::CREATE(_, _)));
    assert!(a0.lines().len() == 1);
    assert!(a0.lines()[0] == "type nat is (int x)");
}

#[test]
fn single_frame_02() {
    // Frame with insert action
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
where x >= 0"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::CREATE(_, _)));
    assert!(a0.lines().len() == 2);
    assert!(a0.lines()[0] == "type nat is (int x)");
    assert!(a0.lines()[1] == "where x >= 0");
}

#[test]
fn single_frame_03() {
    // Frame with multiple insert actions
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
>>> other.whiley
type uint is (int y)"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 2);
    //
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::CREATE(_, _)));
    assert!(a0.lines().len() == 1);
    assert!(a0.lines()[0] == "type nat is (int x)");
    //
    let a1 = &f0.actions[1];
    assert!(a1.lines().len() == 1);
    assert!(a1.lines()[0] == "type uint is (int y)");
}

#[test]
fn single_frame_04() {
    // Frame with remove action
    let wtf = parse(
        r#"
====
<<< main.whiley"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::REMOVE(_)));
}

#[test]
fn single_frame_05() {
    // Whitespace after action line
    let wtf = parse("====\n<<< main.whiley ");
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::REMOVE(_)));
}

#[test]
fn single_frame_06() {
    // Frame with action range
    let wtf = parse(
        r#"
====
>>> main.whiley 0
type nat is (int x)"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::INSERT(_, _, _)));
    assert!(a0.range() == &Range(0, 0));
    assert!(a0.lines().len() == 1);
}

#[test]
fn single_frame_07() {
    // Whitespace after action range
    let wtf = parse("====\n>>> main.whiley 0 \ntype nat is (int x)");
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::INSERT(_, _, _)));
    assert!(a0.range() == &Range(0, 0));
    assert!(a0.lines().len() == 1);
}

#[test]
fn single_frame_08() {
    // Frame with action range
    let wtf = parse(
        r#"
====
>>> main.whiley 0:1
type nat is (int x)"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::INSERT(_, _, _)));
    assert!(a0.range() == &Range(0, 1));
    assert!(a0.lines().len() == 1);
}

#[test]
fn single_frame_09() {
    // Frame with empty markers
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(matches!(a0, Action::CREATE(_, _)));
    assert!(a0.lines().len() == 1);
}

#[test]
fn single_frame_10() {
    // Frame with marker
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,5:7"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 1);
    let m0 = &f0.markers[0];
    assert!(m0.errno == 303);
    assert!(m0.filename == "main.whiley");
    assert!(m0.location == Coordinate(1, Range(5, 7)));
}

#[test]
fn single_frame_11() {
    // Frame with marker
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,5"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 1);
    let m0 = &f0.markers[0];
    assert!(m0.errno == 303);
    assert!(m0.filename == "main.whiley");
    assert!(m0.location == Coordinate(1, Range(5, 5)));
}

#[test]
fn single_frame_12() {
    // Frame with markers
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,5:7
E507 main.whiley 3,3:4"#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 2);
    // First marker
    let m0 = &f0.markers[0];
    assert!(m0.errno == 303);
    assert!(m0.filename == "main.whiley");
    assert!(m0.location == Coordinate(1, Range(5, 7)));
    // Second marker
    let m1 = &f0.markers[1];
    assert!(m1.errno == 507);
    assert!(m1.filename == "main.whiley");
    assert!(m1.location == Coordinate(3, Range(3, 4)));
}

#[test]
fn single_frame_13() {
    // White space after marker
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,5 "#,
    );
    assert!(wtf.size() == 1);
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 1);
    let m0 = &f0.markers[0];
    assert!(m0.errno == 303);
    assert!(m0.filename == "main.whiley");
    assert!(m0.location == Coordinate(1, Range(5, 5)));
}

#[test]
fn single_frame_invalid_01() {
    // Frame with invalid insert action
    let _wtf = parse_expecting(
        r#"
====
>>>
type nat is (int x)"#,
        Error::InvalidAction,
    );
}

#[test]
fn single_frame_invalid_02() {
    // Frame with invalid insert action
    let _wtf = parse_expecting(
        r#"
====
>>> 1 2 3
type nat is (int x)"#,
        Error::InvalidAction,
    );
}

#[test]
fn single_frame_invalid_03() {
    // Frame with invalid remove action
    let _wtf = parse_expecting(
        r#"
====
<<<
type nat is (int x)"#,
        Error::InvalidAction,
    );
}

#[test]
fn single_frame_invalid_04() {
    // Frame with invalid remove action
    let _wtf = parse_expecting(
        r#"
====
<<< 1 2 3
type nat is (int x)"#,
        Error::InvalidAction,
    );
}

#[test]
fn single_frame_invalid_05() {
    // Frame with invalid action range
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley x
type nat is (int x)"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_06() {
    // Frame with invalid action range
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley 1-
type nat is (int x)"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_07() {
    // Frame with invalid action range
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley 1:
type nat is (int x)"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_08() {
    // Frame with invalid action range
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley 1:x
type nat is (int x)"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_09() {
    // Frame with invalid marker
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303"#,
        Error::InvalidMarker,
    );
}

#[test]
fn single_frame_invalid_10() {
    // Frame with invalid marker
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley"#,
        Error::InvalidMarker,
    );
}

#[test]
fn single_frame_invalid_11() {
    // Frame with invalid marker
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 0,1:2 abc"#,
        Error::InvalidMarker,
    );
}

#[test]
fn single_frame_invalid_12() {
    // Frame with invalid marker
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
Exx main.whiley 1,5:7"#,
        Error::InvalidErrorCode,
    );
}

#[test]
fn single_frame_invalid_13() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1:5:7"#,
        Error::InvalidCoordinate,
    );
}

#[test]
fn single_frame_invalid_14() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,5,7"#,
        Error::InvalidCoordinate,
    );
}

#[test]
fn single_frame_invalid_15() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley x,5:7"#,
        Error::InvalidCoordinate,
    );
}

#[test]
fn single_frame_invalid_16() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,x:7"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_17() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,1:"#,
        Error::InvalidRange,
    );
}

#[test]
fn single_frame_invalid_18() {
    // Frame with invalid marker coordinate
    let _wtf = parse_expecting(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E303 main.whiley 1,1:x"#,
        Error::InvalidRange,
    );
}

// ===============================================================
// Multi Frame Tests
// ===============================================================

#[test]
fn multi_frame_01() {
    // Single file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
====
>>> main.whiley
type uint is (int y)"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    let a0 = &f0.actions[0];
    assert!(a0.lines().len() == 1);
    assert!(a0.lines()[0] == "type nat is (int x)");
    //
    let f1 = wtf.frame(1);
    assert!(f1.actions.len() == 1);
    let a1 = &f1.actions[0];
    assert!(a1.lines().len() == 1);
    assert!(a1.lines()[0] == "type uint is (int y)");
}

#[test]
fn multi_frame_02() {
    // Single file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
E101 main.whiley 1,5:7
====
>>> main.whiley
type uint is (int y)"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 1);
    //
    let f1 = wtf.frame(1);
    assert!(f1.markers.len() == 0);
}

#[test]
fn multi_frame_03() {
    // Single file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
====
>>> main.whiley
type uint is (int y)
---
E101 main.whiley 1,5"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 0);
    //
    let f1 = wtf.frame(1);
    assert!(f1.markers.len() == 1);
}

#[test]
fn multi_frame_04() {
    // Single file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
---
====
<<< main.whiley
>>> other.whiley
type uint is (int y)
---
E101 other.whiley 1,5
====
>>> other.whiley
// A comment"#,
    );
    assert!(wtf.size() == 3);
    //
    let f0 = wtf.frame(0);
    assert!(f0.markers.len() == 0);
    //
    let f1 = wtf.frame(1);
    assert!(f1.markers.len() == 1);
}

#[test]
fn multi_frame_05() {
    // Multi file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
>>> other.whiley
type uint is (int x)
====
>>> main.whiley
type uint is (int y)
---
E101 main.whiley 1,5"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 2);
    assert!(f0.markers.len() == 0);
    //
    let f1 = wtf.frame(1);
    assert!(f1.markers.len() == 1);
}

#[test]
fn multi_frame_06() {
    // Multi file
    let wtf = parse(
        r#"
====
>>> main.whiley
type nat is (int x)
>>> other.whiley
// Unsigned int
type uint is (int x)
====
>>> main.whiley
type uint is (int y)
>>> other2.whiley
type uint is (int x)
---
E101 main.whiley 1,5"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 2);
    assert!(f0.markers.len() == 0);
    //
    let f1 = wtf.frame(1);
    assert!(f1.actions.len() == 2);
    assert!(f1.markers.len() == 1);
}

#[test]
fn multi_frame_07() {
    // Multi file
    let wtf = parse(
        r#"
wyc.compile = false
====
>>> main.whiley
type nat is (int x)
====
>>> main.whiley
type uint is (int y)
>>> other2.whiley
type uint is (int x)
---
E101 main.whiley 1,5"#,
    );
    assert!(wtf.size() == 2);
    //
    let f0 = wtf.frame(0);
    assert!(f0.actions.len() == 1);
    assert!(f0.markers.len() == 0);
    //
    let f1 = wtf.frame(1);
    assert!(f1.actions.len() == 2);
    assert!(f1.markers.len() == 1);
}

// ===============================================================
// Helpers
// ===============================================================

fn parse(input: &str) -> WhileyTestFile {
    // Parser test file
    let wtf = WhileyTestFile::new(input);
    // Assume parsing succeeded
    assert!(wtf.is_ok());
    // Done
    wtf.unwrap()
}

fn parse_config_option(input: &str, key: &str, val: Value) {
    // Parser test file
    let wtf = WhileyTestFile::new(input).unwrap();
    // Look for value
    match wtf.get(key) {
        Some(v) => {
            if &val != v {
                panic!(
                    "Expecting config value {:?} for key {:?}, got {:?}!",
                    val, key, v
                );
            }
        }
        _ => {
            panic!(
                "Expected config value {:?} for key {:?}, got nothing!",
                val, key
            );
        }
    }
}

fn parse_expecting(input: &str, expected: Error) {
    // Parser test file
    match WhileyTestFile::new(input) {
        Ok(_) => {
            panic!("File should not have parsed!");
        }
        Err(err) => {
            if err != expected {
                panic!("Expected error {:?}, got {:?}!", expected, err);
            }
        }
    }
}
