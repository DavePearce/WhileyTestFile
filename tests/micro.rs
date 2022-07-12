use whiley_test_file::WhileyTestFile;

#[test]
fn test_01() {
    let wtf = WhileyTestFile::from_str("hello\n another");
}
