use crate::parsers::*;
use crate::regexes::*;
use regex::Regex;


/// A parser that that succeed if the given regex matches the input.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
/// use parser_combinator::regexes::*;
///
/// let parser = matching(&JSON_NUMBER_REGEX);
/// assert_eq!(Ok(("", "-2.4")), parser.parse("-2.4"));
/// assert!(parser.parse("NaN").is_err());
/// ```
pub const fn matching<'a, 'b>(
    expected: &Regex,
) -> impl Parser<&'a str, &'b str, &'b str, String> + '_
where
    'a: 'b,
{
    move |input: &'a str| match expected.find(&input[..]) {
        Some(matched) => Ok((
            &input[matched.end()..],
            &input[matched.start()..matched.end()],
        )),

        None => Err(format!("Could not parse '{}'", expected.as_str())),
    }
}

/// Matches exactly the given word but insensitive to case.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = literal(" foo");
/// assert_eq!(Ok(("", " foo")), parser.parse(" foo"));
/// assert!(parser.parse("foo").is_err());
/// ```
pub const fn literal<'a, 'b, A>(expected: A) -> impl Parser<&'a str, &'b str, &'b str, String>
where
    A: AsRef<str>,
    'a: 'b,
{
    move |input: &'a str| {
        let expected = expected.as_ref();
        if input.len() < expected.len() {
            return Err(format!("Could not parse '{}'", expected));
        }
        match &input[0..expected.len()].to_lowercase() {
            ex if ex == &expected.to_lowercase() => {
                Ok((&input[expected.len()..], &input[0..expected.len()]))
            }

            _ => Err(format!("Could not parse '{}'", expected)),
        }
    }
}

/// Parse an identifier, to most programming languages sense.
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("", "foo_a1")), identifier.parse("foo_a1"));
/// ```
pub fn identifier<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&IDENT_REGEX).parse(input)
}

/// Parse any type of whitespace.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("foo_a1", " ")), whitespace.parse(" foo_a1"));
/// ```
pub fn whitespace<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&WHITESPACE_REGEX).parse(input)
}

/// Parse any type of whitespace that is not a newline.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("\n", " ")), whitespace_no_newline.parse(" \n"));
/// ```
pub fn whitespace_no_newline<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&WHITESPACE_NO_NEWLINE_REGEX).parse(input)
}

/// Parse multiple consecutive newline characters.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok((" ", "\n\n")), newline.parse("\n\n "));
/// ```
pub fn newline<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&NEWLINE_REGEX).parse(input)
}

/// Matches exactly one character that could be a newline.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("\n", "\n")), one_newline.parse("\n\n"));
/// ```
pub fn one_newline<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    literal("\n").parse(input)
}

/// Matches any character that is not a unicode whitespace.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("\n ", "a#@^%$6")), not_whitespace.parse("a#@^%$6\n "));
/// ```
pub fn not_whitespace<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&NOT_WHITESPACE_REGEX).parse(input)
}

/// Matches any character that does not mean newline (\n and \r).
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// assert_eq!(Ok(("\n", "a#@^%$6 ")), not_newline.parse("a#@^%$6 \n"));
/// ```
pub fn not_newline<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    matching(&NOT_NEWLINE_REGEX).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_whitespace_no_newline() {
        let result = whitespace_no_newline.parse(" \t\r\n");
        assert_eq!(Ok(("\r\n", " \t")), result);
    }

    #[test]
    fn test_whitespace_no_newline_2() {
        let result = whitespace_no_newline.parse("\r\n\r\n");
        assert!(result.is_err());
    }

    #[test]
    fn test_newline() {
        assert!(newline.parse("\t").is_err());
        assert!(newline.parse(" ").is_err());

        assert!(newline.parse("\n").is_ok());
        assert!(newline.parse("\r\n").is_ok());
    }

    #[test]
    fn test_newline_2() {
        assert_eq!(newline.parse("\n\t"), Ok(("\t", "\n")));
    }
}
