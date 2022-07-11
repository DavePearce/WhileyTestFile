use std::collections::HashMap;
use std::result;

// ===============================================================
// Error
// ===============================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Error {
    UnexpectedEof
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
        todo!["Implement me"]
    }
}

// ===============================================================
// Config
// ===============================================================

enum Object {
    String(String),
    Int(u64),
    Bool(bool)
}
type Config = HashMap<String, Object>;

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
