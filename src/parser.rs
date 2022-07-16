use crate::{Action,ActionKind,Config,Error,Frame,Range,Result,WhileyTestFile,Value};

pub struct Parser<'a> {
    // Identifies current line number.
    index: usize,
    // Holds the set of lines.
    lines: Vec<&'a str>
}

impl<'a> Parser<'a> {

    /// Construct a new parser from a given string slice.
    pub fn new(input: &'a str) -> Self {
        let lines = input.lines().collect();
        Parser{index:0, lines}
    }

    // ===============================================================
    // Public API
    // ===============================================================

    /// Parse configuration from this point
    pub fn parse(&mut self) -> Result<WhileyTestFile> {
        let config = self.parse_config()?;
        let frames = self.parse_frames()?;
        Ok(WhileyTestFile{config,frames})
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
        self.index = self.index + 1;
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
            if line != "" {
                let (k,v) = parse_kvp_line(line)?;
                config.insert(k,v);
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

	// TODO
	
	Ok(Frame{actions,markers})
    }

    fn parse_action(&mut self) -> Result<Action> {
	// Split action header by spaces.
	let line : Vec<&str> = self.next().split(" ").collect();
	// Determine action kind
	let kind = if line[0] == ">>>" {
	    ActionKind::INSERT
	} else { ActionKind::REMOVE };
	// Parse filename and (optional) range.
	let (filename,range) = match line.len() {
	    2 => (line[1].to_string(), None),
	    3 => {
		(line[1].to_string(), Some(parse_range(line[2])?))
	    }
	    _ => {
		return Err(Error::InvalidAction);
	    }
	};
	// Parse action content
	let mut lines = Vec::new();
	while !self.eof() && !is_prefix(self.peek()) {
	    lines.push(self.next().to_string());
        }	
	// Done
	Ok(Action{kind,filename,range,lines})
    }
}

/// Parse a line of text containing a key-value assignment, such as:
///
/// ```text
/// wyc.compile = false
/// ```
fn parse_kvp_line(line: &str) -> Result<(String,Value)> {
    // Split line into components
    let bits : Vec<&str> = line.split('=').collect();
    // Sanity check only two components!
    if bits.len() != 2 {
        // Something is wrong!
        Err(Error::InvalidConfigOption)
    } else {
        let key = bits[0].trim().to_string();
        let value = parse_object(bits[1].trim())?;
        Ok((key,value))
    }
}

/// Parse a configuration object.
fn parse_object(input: &str) -> Result<Value> {
    // Extract first character.
    let c = input.chars().next();
    //
    match c {
        // Match ASCII digit.
        Some('0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-') => {
            parse_int_object(input)
        }
        Some('"') => {
            parse_string_object(input)
        }
        _ => {
            parse_bool_object(input)
        }
    }
}

/// Parse a string which should represent a (signed) integer value.
/// If parsing fails for some reason, return appropriate error.
fn parse_int_object(input: &str) -> Result<Value> {
    match input.parse::<i64>() {
        Ok(i) => Ok(Value::Int(i)),
        _ => Err(Error::InvalidIntValue)
    }
}

fn parse_string_object(input: &str) -> Result<Value> {
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

fn parse_bool_object(input: &str) -> Result<Value> {
    if input == "true" {
        Ok(Value::Bool(true))
    } else if input == "false" {
        Ok(Value::Bool(false))
    } else {
        return Err(Error::InvalidConfigValue)
    }
}

/// Parse a "range" which is either a single unsigned integer
/// (e.g. `1`), or a pair of unsigned ints separated by a colon
/// (e.g. `0:2`).
fn parse_range(input: &str) -> Result<Range> {
    let split : Vec<&str> = input.split(":").collect();
    // Match the kind of range we have.
    match split.len() {
	1 => {
	    let i = parse_range_index(split[0])?;
	    Ok(Range(i,i))
	}
	2 => {
	    let i = parse_range_index(split[0])?;
	    let j = parse_range_index(split[1])?;	    
	    Ok(Range(i,j))
	}
	_ => {
	    return Err(Error::InvalidRange);
	}
    }
}

/// Parse an unsigned int which forms part of some range.  In essence,
/// this method just handles the mapping of error values.
fn parse_range_index(input: &str) -> Result<usize> {
    match input.parse::<usize>() {
	Ok(i) => Ok(i),
	_ => Err(Error::InvalidRange)
    }
}

/// Determine whether the given string (which represents a line)
/// begins with one of the key control markers (e.g. `===` which
/// indicates the start of a frame, etc).
fn is_prefix(line: &str) -> bool {
    return line.starts_with("===")
	|| line.starts_with("---")
	|| is_action_prefix(line);
}

/// Determine whether the given string (which represents a line)
/// identifies the start of an action.
fn is_action_prefix(line: &str) -> bool {
    return line.starts_with(">>>") || line.starts_with("<<<");
}