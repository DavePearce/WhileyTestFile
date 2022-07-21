use std::fs;
use std::path::{Path,PathBuf};
use whiley_test_file::{WhileyTestFile};

pub static REFTESTS_DIR : &'static str = "reference-tests/tests";

/// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/reftests.rs"));

/// Run a specific test by loading the file out of the reference tests
/// repository and attempting to parse it.  All reference tests should
/// parse correctly.
fn check(test: &str) {
    // Construct filename
    let mut path = PathBuf::from(REFTESTS_DIR);
    path.push(test);
    let mut filename = path.as_path().to_str().unwrap();
    // Read the test file
    let input = fs::read_to_string(filename).unwrap();
    // Parser test file
    let wtf = WhileyTestFile::from_str(&input);
    // Assume parsing succeeded
    assert!(wtf.is_ok());
}
