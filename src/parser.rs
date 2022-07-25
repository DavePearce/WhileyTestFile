use crate::{
    Action, Config, Coordinate, Error, Frame, Marker, Range, Result, Value, WhileyTestFile,
};

pub struct Parser<'a> {
    // Identifies current line number.
    index: usize,
    // Holds the set of lines.
    lines: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    /// Construct a new parser from a given string slice.
    pub fn new(input: &'a str) -> Self {
        let lines = input.lines().collect();
        Parser { index: 0, lines }
    }

    // ===============================================================
    // Public API
    // ===============================================================

    /// Parse configuration from this point
    pub fn parse(&mut self) -> Result<WhileyTestFile> {
        let config = self.parse_config()?;
        let frames = self.parse_frames()?;
        Ok(WhileyTestFile { config, frames })
    }

    /// Check whether end-of-file reached.
    pub fn eof(&self) -> bool {
        self.index >= self.lines.len()
    }

    /// Peek at next line in the file.
    pub fn peek(&self) -> &'a str {
        assert!(!self.eof());
        self.lines[self.index]
    }

    pub fn next(&mut self) -> &'a str {
        assert!(!self.eof());
        let line = self.lines[self.index];
        self.index += 1;
        line
    }

    // ===============================================================
    // Private Helpers
    // ===============================================================

    /// Parse configuration from this point
    fn parse_config(&mut self) -> Result<Config> {
        let mut config = Config::new();
        // Continue parsing until start of first frame.
        while !self.eof() && !self.peek().starts_with("===") {
            let line = self.next().trim();
            // Skip empty lines.
            if !line.is_empty() {
                let (k, v) = parse_kvp_line(line)?;
                config.insert(k, v);
            }
        }
        Ok(config)
    }

    /// Parse frames from this point
    fn parse_frames(&mut self) -> Result<Vec<Frame>> {
        let mut frames = Vec::new();
        // Parse as many frames as there are.
        while !self.eof() && self.peek().starts_with("===") {
            frames.push(self.parse_frame()?);
        }
        Ok(frames)
    }

    fn parse_frame(&mut self) -> Result<Frame> {
        let _l = self.next(); // skip line beginning "==="
        let mut actions = Vec::new();
        // Parse actiondelta's
        while !self.eof() && is_action_prefix(self.peek()) {
            actions.push(self.parse_action()?);
        }
        // Parse any markers
        let mut markers = Vec::new();
        if !self.eof() && is_marker_prefix(self.peek()) {
            self.next(); // skip prefix
            while !self.eof() && !is_prefix(self.peek()) {
                markers.push(self.parse_marker()?);
            }
        }
        // Done
        Ok(Frame { actions, markers })
    }

    fn parse_action(&mut self) -> Result<Action> {
        let line = self.next().trim();
        // Split action header by spaces.
        let split: Vec<&str> = line.split(' ').collect();
        // Parse filename and (optional) range.
        let (filename, range) = match split.len() {
            2 => (split[1].to_string(), None),
            3 => (split[1].to_string(), Some(parse_range(split[2])?)),
            _ => {
                return Err(Error::InvalidAction);
            }
        };
        // Parse action content
        let mut lines = Vec::new();
        while !self.eof() && !is_prefix(self.peek()) {
            lines.push(self.next().to_string());
        }
        // Determine action kind
        let act = if split[0] == ">>>" {
            match range {
                Some(r) => Action::INSERT(filename, r, lines),
                None => Action::CREATE(filename, lines),
            }
        } else {
            Action::REMOVE(filename)
        };
        Ok(act)
    }

    /// Parser a marker which identifies something with a given
    /// position in the file (e.g. an error code associated with a
    /// given line and column in the file.
    fn parse_marker(&mut self) -> Result<Marker> {
        let line = self.next().trim();
        // Split line into components
        let split: Vec<&str> = line.split(' ').collect();
        // Sanity check enough components
        if split.len() == 3 {
            let errno = parse_error_code(split[0])?;
            let filename = split[1].to_string();
            let location = parse_coordinate(split[2])?;
            Ok(Marker {
                errno,
                filename,
                location,
            })
        } else {
            Err(Error::InvalidMarker)
        }
    }
}

/// Parse a line of text containing a key-value assignment, such as:
///
/// ```text
/// wyc.compile = false
/// ```
fn parse_kvp_line(line: &str) -> Result<(String, Value)> {
    // Split line into components
    let bits: Vec<&str> = line.split('=').collect();
    // Sanity check only two components!
    if bits.len() != 2 {
        // Something is wrong!
        Err(Error::InvalidConfigOption)
    } else {
        let key = bits[0].trim().to_string();
        let value = parse_value(bits[1].trim())?;
        Ok((key, value))
    }
}

/// Parse a configuration object.
fn parse_value(input: &str) -> Result<Value> {
    // Extract first character.
    let c = input.chars().next();
    //
    match c {
        // Match ASCII digit.
        Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-') => {
            parse_int_value(input)
        }
        Some('"') => parse_string_value(input),
        _ => parse_bool_value(input),
    }
}

/// Parse a string which should represent a (signed) integer value.
/// If parsing fails for some reason, return appropriate error.
fn parse_int_value(input: &str) -> Result<Value> {
    match input.parse::<i64>() {
        Ok(i) => Ok(Value::Int(i)),
        _ => Err(Error::InvalidIntValue),
    }
}

fn parse_string_value(input: &str) -> Result<Value> {
    let n = input.len() - 1;
    // Check last element is quote
    if n > 0 && &input[n..] == "\"" {
        // Extract content
        let content = &input[1..n];
        // Sanity check quotes don't appear within.
        if !content.contains('"') {
            // Success
            return Ok(Value::String(content.to_string()));
        }
    }
    Err(Error::InvalidStringValue)
}

fn parse_bool_value(input: &str) -> Result<Value> {
    if input == "true" {
        Ok(Value::Bool(true))
    } else if input == "false" {
        Ok(Value::Bool(false))
    } else {
        Err(Error::InvalidConfigValue)
    }
}

/// Parse a "coodinate" which identifies a character range within a
/// given line.  For example, `1,0:2` identifies the range `0:2`
/// within line `1`.
fn parse_coordinate(input: &str) -> Result<Coordinate> {
    let split: Vec<&str> = input.split(',').collect();
    // Sanity check sufficient components
    if split.len() == 2 {
        let line = parse_coordinate_index(split[0])?;
        let range = parse_range(split[1])?;
        Ok(Coordinate(line, range))
    } else {
        Err(Error::InvalidCoordinate)
    }
}

/// Parse an unsigned int which forms part of some coordinate.  In
/// essence, this method just handles the mapping of error values.
fn parse_coordinate_index(input: &str) -> Result<usize> {
    match input.parse::<usize>() {
        Ok(i) => Ok(i),
        _ => Err(Error::InvalidCoordinate),
    }
}

/// Parse a "range" which is either a single unsigned integer
/// (e.g. `1`), or a pair of unsigned ints separated by a colon
/// (e.g. `0:2`).
fn parse_range(input: &str) -> Result<Range> {
    let split: Vec<&str> = input.split(':').collect();
    // Match the kind of range we have.
    match split.len() {
        1 => {
            let i = parse_range_index(split[0])?;
            Ok(Range(i, i))
        }
        2 => {
            let i = parse_range_index(split[0])?;
            let j = parse_range_index(split[1])?;
            Ok(Range(i, j))
        }
        _ => Err(Error::InvalidRange),
    }
}

/// Parse an unsigned int which forms part of some range.  In essence,
/// this method just handles the mapping of error values.
fn parse_range_index(input: &str) -> Result<usize> {
    match input.parse::<usize>() {
        Ok(i) => Ok(i),
        _ => Err(Error::InvalidRange),
    }
}

/// Parse an error code which is an identifier followed by an unsigned
/// int (e.g. `E101`, `W23`, etc).
fn parse_error_code(mut input: &str) -> Result<u16> {
    input = &input[1..];
    match input.parse::<u16>() {
        Ok(i) => Ok(i),
        _ => Err(Error::InvalidErrorCode),
    }
}

/// Determine whether the given string (which represents a line)
/// begins with one of the key control markers (e.g. `===` which
/// indicates the start of a frame, etc).
fn is_prefix(line: &str) -> bool {
    is_frame_prefix(line) || is_action_prefix(line) || is_marker_prefix(line)
}

/// Determine whether the given string (which represents a line)
/// identifies the start of a framer.
fn is_frame_prefix(line: &str) -> bool {
    line.starts_with("===")
}

/// Determine whether the given string (which represents a line)
/// identifies the start of an action.
fn is_action_prefix(line: &str) -> bool {
    line.starts_with(">>>") || line.starts_with("<<<")
}

/// Determine whether the given string (which represents a line)
/// identifies the start of a marker block.
fn is_marker_prefix(line: &str) -> bool {
    line.starts_with("---")
}
