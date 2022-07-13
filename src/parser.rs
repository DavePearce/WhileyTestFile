use crate::{Config,Error,Frame,Object,Result,WhileyTestFile};

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
        while !self.eof() && self.peek() != ">>>" {
            let line = self.next();
            let (k,v) = parse_kvp_line(line)?;
            config.insert(k,v);
        }
        Ok(Config::new())
    }

    /// Parse frames from this point
    fn parse_frames(&mut self) -> Result<Vec<Frame>> {
        Ok(Vec::new())
    }
}

/// Parse a line of text containing a key-value assignment, such as:
///
/// ```text
/// wyc.compile = false
/// ```
fn parse_kvp_line(line: &str) -> Result<(String,Object)> {
    // Split line into components
    let bits : Vec<&str> = line.split('=').collect();
    // Sanity check only two components!
    if bits.len() != 2 {
        // Something is wrong!
        Err(Error::InvalidConfigOption)
    } else {
        let key = bits[0].to_string();
        let value = parse_object(bits[1].trim())?;
        println!("Parsed: {} = {:?}", line, value);
        Ok((key,value))
    }
}

/// Parse a configuration object.
fn parse_object(input: &str) -> Result<Object> {
    // Extract first character.
    let c = input.chars().next();
    //
    match c {
        // Match ASCII digit.
        Some('0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9') => {
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

fn parse_int_object(input: &str) -> Result<Object> {
    //let i : i64 = i64::parse(input);
    Ok(Object::Int(1))
}

fn parse_string_object(input: &str) -> Result<Object> {
    let chars : Vec<char> = input.chars().collect();
    // Sanity check that it ends with a '"', and none inbetween!
    Ok(Object::String("".to_string()))
}

fn parse_bool_object(input: &str) -> Result<Object> {
    if input == "true" {
        Ok(Object::Bool(true))
    } else if input == "false" {
        Ok(Object::Bool(false))
    } else {
        return Err(Error::InvalidConfigValue)
    }
}
