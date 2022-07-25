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
//! use std::str::FromStr;
//! use whiley_test_file::WhileyTestFile;
//!
//! fn load(filename: &str) -> WhileyTestFile {
//!     // Read the test file
//!     let input = fs::read_to_string(filename).unwrap();
//!     // Parse test file
//!     return WhileyTestFile::from_str(&input).unwrap()
//! }
//! ```

// Hidden modules
mod parser;

use parser::Parser;
use std::collections::HashMap;
use std::result;
use std::str::FromStr;

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

pub struct WhileyTestFile {
    config: Config,
    frames: Vec<Frame>,
}

impl WhileyTestFile {
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
    pub fn get_str(&self, key: &str) -> Option<&String> {
        match &self.config.get(key) {
            Some(&Value::String(ref s)) => Some(s),
            _ => None,
        }
    }
}

impl FromStr for WhileyTestFile {
    type Err = Error;

    fn from_str(input: &str) -> Result<WhileyTestFile> {
        // Construct parser
        let mut parser = Parser::new(input);
        // Parse file (with errors)
        let wtf = parser.parse()?;
        // Done
        Ok(wtf)
    }
}

// ===============================================================
// Config
// ===============================================================

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Bool(bool),
}
type Config = HashMap<String, Value>;

// ===============================================================
// Frame
// ===============================================================

/// Represents a frame within a testfile.  Each frame identifies a
/// number of _actions_ which operate on the state at that point,
/// along with zero or more expected _markers_ (e.g. error messages).
/// The set of actions includes _inserting_ and _removing_ lines on a
/// specific file.  Actions are applied in the order of appearance,
/// though they are not expected to overlap.
pub struct Frame {
    pub actions: Vec<Action>,
    pub markers: Vec<Marker>,
}

// ===============================================================
// Action
// ===============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionKind {
    INSERT,
    REMOVE,
}

/// Represents an atomic action which can be applied to a source file,
/// such as inserting or replacing lines within the file.
pub struct Action {
    pub kind: ActionKind,
    pub filename: String,
    pub range: Option<Range>,
    pub lines: Vec<String>,
}

// ===============================================================
// Marker
// ===============================================================

/// Identifies an expected error at a location in a given source file.
pub struct Marker {
    pub errno: u16,
    pub filename: String,
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
