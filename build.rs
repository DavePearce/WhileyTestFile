use std::fs;
use std::io::Write;

pub static REFTESTS_DIR: &str = "reference-tests/tests";

/// The purpose of this script is to generate a set of tests for each
/// of the language reference tests.
fn main() {
    // Create destination file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("reftests.rs");
    let mut f = fs::File::create(&destination).unwrap();
    // Open reference test directory
    let dir = fs::read_dir(REFTESTS_DIR).unwrap();

    for e in dir {
        let p = e.as_ref().unwrap().path();
        let n = p.file_stem().unwrap().to_str().unwrap();
        if p.extension().unwrap() == "test" {
            write!(
                f,
                "
#[test]
fn test_{name}() {{
    check(\"{name}.test\");
}}",
                name = n
            )
            .unwrap();
        }
    }
}
