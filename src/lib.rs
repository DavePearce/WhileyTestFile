//! A library for parsing Whiley test files according to
//! [RFC#110](https://github.com/Whiley/RFCs/blob/master/text/0110-test-file-format.md)
//! which are used for testing the [Whiley
//! compiler](https://github.com/Whiley/WhileyCompiler).  Each test
//! describes a sequence of modifications to one of more Whiley files,
//! along with the expected outcomes (e.g. errors, warnings, etc).  An
//! example test file is the following:
//! ```text
//! whiley.verify = false
//! boogie.timeout = 1000
//! ================
//! >>> main.whiley
//! method main():
//! >>> other.whiley
//! import main
//! ---
//! E101 main.whiley 1,2
//! E302 main.whiley 2,2:3
//! ================
//! <<< other.whiley
//! >>> main.whiley 1:1
//! method main()
//!     skip
//! ---
//! ```
//! This is a test involving two files: `main.whiley` and `other.whiley`.
//! The initial frame sets the contents of `main.whiley` to `method
//! main()` and the contents of `other.whiley` to `import main`.
//! Furthermore, compiling this frame is expected to produce two errors
//! (`E101` and `E302`).  The second frame deletes file `other.whiley` and
//! updates the contents of `main.whiley`.  Furthermore, compiling the
//! snapshot at this point is not expected to produce any errors.
//!
//! ```
//! use std::fs;
//! use whiley_test_file::WhileyTestFile;
//!
//! fn load(filename: &str) {
//!     // Read the test file
//!     let input = fs::read_to_string(filename).unwrap();
//!     // Parse test file
//!     let test_file = WhileyTestFile::new(&input).unwrap();
//!     // ...
//! }
//! ```

// Hidden modules
mod parser;

use parser::Parser;
use std::collections::HashMap;
use std::result;

// ===============================================================
// Error
// ===============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidConfigOption,
    InvalidConfigValue,
    InvalidIntValue,
    InvalidStringValue,
    InvalidAction,
    InvalidRange,
    InvalidMarker,
    InvalidErrorCode,
    InvalidCoordinate,
}

pub type Result<T> = result::Result<T, Error>;

// ===============================================================
// Test File
// ===============================================================

pub struct WhileyTestFile<'a> {
    config: Config<'a>,
    frames: Vec<Frame<'a>>,
}

impl<'a> WhileyTestFile<'a> {
    pub fn new(input: &'a str) -> Result<WhileyTestFile<'a>> {
        // Construct parser
        let mut parser = Parser::new(input);
        // Parse file (with errors)
        let wtf = parser.parse()?;
        // Done
        Ok(wtf)
    }

    /// Get configuration option associated with the given key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.config.get(key)
    }

    /// Get number of frames in this test file.
    pub fn size(&self) -> usize {
        self.frames.len()
    }

    /// Get nth frame within this test file.
    pub fn frame(&self, n: usize) -> &Frame {
        &self.frames[n]
    }

    /// Get configuration option which is expected to be an integer.
    /// If its not an integer, or no such key exists, `None` is
    /// returned.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        match self.config.get(key) {
            Some(&Value::Int(i)) => Some(i),
            _ => None,
        }
    }

    /// Get configuration option which is expected to be an boolean.
    /// If its not a boolean, or no such key exists, `None` is
    /// returned.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.config.get(key) {
            Some(&Value::Bool(b)) => Some(b),
            _ => None,
        }
    }

    /// Get configuration option which is expected to be a string If
    /// its not a string, or no such key exists, `None` is returned.
    pub fn get_str(&self, key: &str) -> Option<&'a str> {
        match &self.config.get(key) {
            Some(&Value::String(s)) => Some(s),
            _ => None,
        }
    }
}

// ===============================================================
// Config
// ===============================================================

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Int(i64),
    Bool(bool),
}
type Config<'a> = HashMap<&'a str, Value<'a>>;

// ===============================================================
// Frame
// ===============================================================

/// Represents a frame within a testfile.  Each frame identifies a
/// number of _actions_ which operate on the state at that point,
/// along with zero or more expected _markers_ (e.g. error messages).
/// The set of actions includes _inserting_ and _removing_ lines on a
/// specific file.  Actions are applied in the order of appearance,
/// though they are not expected to overlap.
pub struct Frame<'a> {
    pub actions: Vec<Action<'a>>,
    pub markers: Vec<Marker<'a>>,
}

// ===============================================================
// Action
// ===============================================================

/// Represents an atomic action which can be applied to a source file,
/// such as inserting or replacing lines within the file.
#[derive(Debug, PartialEq)]
pub enum Action<'a> {
    CREATE(&'a str, Vec<&'a str>),
    REMOVE(&'a str),
    INSERT(&'a str, Range, Vec<&'a str>),
}

impl<'a> Action<'a> {
    pub fn lines(&self) -> &[&'a str] {
        match self {
            Action::CREATE(_, lines) => lines,
            Action::INSERT(_, _, lines) => lines,
            _ => {
                panic!("no line information!");
            }
        }
    }

    pub fn range(&self) -> &Range {
        match self {
            Action::INSERT(_, r, _) => r,
            _ => {
                panic!("no range information!");
            }
        }
    }
}

// ===============================================================
// Marker
// ===============================================================

/// Identifies an expected error at a location in a given source file.
pub struct Marker<'a> {
    pub errno: u16,
    pub filename: &'a str,
    pub location: Coordinate,
}

// ===============================================================
// Coordinate
// ===============================================================

/// Identifies a specific range of characters within a file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate(pub usize, pub Range);

// ===============================================================
// Range
// ===============================================================

/// Represents an interval (e.g. of characters within a line).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Range(pub usize, pub usize);
