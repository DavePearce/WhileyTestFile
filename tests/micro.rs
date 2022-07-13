use whiley_test_file::WhileyTestFile;

// ===============================================================
// Config Tests
// ===============================================================

#[test]
fn test_config() {
    let wtf = parse("hello = 1");
}

// ===============================================================
// Helpers
// ===============================================================

fn parse(input: &str) -> WhileyTestFile {
    // Parser test file
    let wtf = WhileyTestFile::from_str(input);
    // Assume parsing succeeded
    assert!(wtf.is_ok());
    //
    wtf.unwrap()
}
