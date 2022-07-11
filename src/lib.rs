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
// Whiley Test File
// ===============================================================

pub struct WhileyTestFile {
    
}

impl WhileyTestFile {
    pub fn from_str<'a>(input: &'a str) -> Result<WhileyTestFile> {
        todo!["Implement me"]
    }
}
