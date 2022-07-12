pub struct Parser<'a> {
    pub lines: Vec<&'a str>
}

impl<'a> Parser<'a> {
    /// Construct a new parser from a given string slice.
    pub fn new(input: &'a str) -> Self {
        let lines = input.lines().collect();
        Parser{lines}
    }
}
