use regex::Regex;

// Regex store for the terminals of the PEG. They essentially recognize strings of terminals in the
// source input. They are compiled only once to save on performance.
lazy_static! {
    /// Regex matching an identifier in the general programming sense.
    pub static ref IDENT_REGEX: Regex = Regex::new(r"\A[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    /// Regex matching any kind of whitespace.
    pub static ref WHITESPACE_REGEX: Regex = Regex::new(r"\A\s+").unwrap();

    /// Regex matching any kind of whitespace except newlines.
    pub static ref WHITESPACE_NO_NEWLINE_REGEX: Regex = Regex::new(r"\A[^\S\r\n]+").unwrap();

    /// Regex matching specifically newlines.
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\A[\r\n]+").unwrap();

    /// Regex Matching any kind of whitespace except newlines.
    pub static ref NOT_NEWLINE_REGEX: Regex = Regex::new(r"\A^[^\n\r]+").unwrap();

    /// Parses any chain of characters except any whitespaces.
    pub static ref NOT_WHITESPACE_REGEX: Regex = Regex::new(r"\A^[^\s]+").unwrap();

    /// Regex for a json `number` terminal.
    pub static ref JSON_NUMBER_REGEX: Regex =
        Regex::new(r"\A-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?").unwrap();
}
