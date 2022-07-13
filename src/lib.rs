// Hidden modules
mod parser;

use std::collections::HashMap;
use std::result;
use parser::Parser;

// ===============================================================
// Error
// ===============================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidConfigOption,
    InvalidConfigValue,
    InvalidIntValue,
    InvalidStringValue
}

pub type Result<T> = result::Result<T, Error>;

// ===============================================================
// Test File
// ===============================================================

pub struct WhileyTestFile {
    config: Config,
    frames: Vec<Frame>
}

impl WhileyTestFile {
    pub fn from_str<'a>(input: &'a str) -> Result<WhileyTestFile> {
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

    /// Get configuration option which is expected to be an integer.
    /// If its not an integer, or no such key exists, `None` is
    /// returned.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        match self.config.get(key) {
            Some(&Value::Int(i)) => Some(i),
            _ => None
        }
    }

    /// Get configuration option which is expected to be an boolean.
    /// If its not a boolean, or no such key exists, `None` is
    /// returned.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.config.get(key) {
            Some(&Value::Bool(b)) => Some(b),
            _ => None
        }
    }

    /// Get configuration option which is expected to be a string If
    /// its not a string, or no such key exists, `None` is returned.
    pub fn get_str(&self, key: &str) -> Option<&String> {
        match &self.config.get(key) {
            Some(&Value::String(ref s)) => Some(s),
            _ => None
        }
    }
}

// ===============================================================
// Config
// ===============================================================

#[derive(Clone,Debug,PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Bool(bool)
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
    actions: Vec<Action>,
    markers: Vec<Marker>
}

// ===============================================================
// Action
// ===============================================================

pub enum ActionKind {
    INSERT,
    REMOVE
}

/// Represents an atomic action which can be applied to a source file,
/// such as inserting or replacing lines within the file.
pub struct Action {
    pub kind: ActionKind,
    pub filename: String,
    pub range: Range,
    pub lines: Vec<String>
}

// ===============================================================
// Marker
// ===============================================================

/// Identifies an expected error at a location in a given source file.
pub struct Marker {
    pub errno: u16,
    pub filename: String,
    //pub location: Coordinate
}

// ===============================================================
// Range
// ===============================================================

/// Represents an interval (e.g. of characters within a line).
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize
}
