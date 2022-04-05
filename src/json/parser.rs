use super::*;
use crate::parsers::*;
use crate::regexes::*;

/// Parse a slice representing Json into a `JsonAst`.
///
/// Fails if the json is not valid
///
/// # Example
/// ```
///  use parser_combinator::json::*;
///
///  assert_eq!(
///     Ok(Json::Array {
///         elem: vec!(
///             Json::String { elem: "bar" },
///             Json::String { elem: "foo" },
///             Json::True { elem: "true" },
///             Json::Object {
///                 elem: vec!((Json::String { elem: "name" }, Json::String { elem: "bob" }))
///             }
///         )
///     }),
///     json("[\"bar\", \"foo\", true, {\"name\": \"bob\"}]")
///  );
/// ```
pub fn json(source: &str) -> Result<Json, String> {
    or(object, array).parse(source).map(|(_, json)| json)
}

/// Parse any terminal value. Terminal values are values that are not recursive json data.
pub const fn value<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // Use fastest failing derivation first
    let parser = or(object, array);
    let parser = or(parser, terminal_value());
    parser
}

/// Parse a Json object.
///
/// A Json object is a series of 'key: value' pair encased in '{ }'.
///
/// # Note
/// Object is a concrete parser instead of a combined parser in order
/// to break type recursion.
pub fn object<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, Json<'b>), String> {
    let middle_pair = left(key_value_pair(), literal(","));
    let middle_pair = left(middle_pair, maybe(whitespace));

    let content = while_(or(middle_pair, key_value_pair()));
    let content = right(maybe(whitespace), content);

    let parser = middle(literal("{"), content, literal("}"));
    let parser = left(parser, maybe(whitespace));

    map(parser, |elem| Json::Object { elem }).parse(input)
}

/// Parse a Json Array.
///
/// A Json array is a series of 'value' pair encased in '[ ]'.
///
/// # Note
/// Array is a concrete parser instead of a combined parser in order to break type recursion.
pub fn array<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, Json<'b>), String> {
    let last_value = right(maybe(whitespace), value());
    let last_value = left(last_value, maybe(whitespace));

    let middle_value = right(maybe(whitespace), value());
    let middle_value = left(middle_value, literal(","));
    let middle_value = left(middle_value, maybe(whitespace));

    let array_value = or(middle_value, last_value);
    let array_content = while_(array_value);

    let parser = middle(literal("["), array_content, literal("]"));
    let parser = left(parser, maybe(whitespace));

    map(parser, |elem| Json::Array { elem }).parse(input)
}

/// Parse a Json object key value pair
///
/// # Note
/// Defined as a private top level function to avoid using moved value in the object parser.
const fn key_value_pair<'a>() -> impl Parser<&'a str, &'a str, (Json<'a>, Json<'a>), String> {
    let key = left(string(), maybe(whitespace));
    let key = left(key, literal(":"));
    let key = left(key, maybe(whitespace));

    let parser = and(key, value());

    parser
}

/// Parse any terminal value.
///
/// Terminal values are values that are not recursive json data.
pub const fn terminal_value<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // Use fastest to fail derivation first
    let parser = or(string(), number());
    let parser = or(parser, true_());
    let parser = or(parser, false_());
    let parser = or(parser, null_());

    // Consume whitespaces after all terminal values
    let parser = left(parser, maybe(whitespace));

    parser
}

/// Parses a `true` terminal.
pub const fn true_<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // NOTE: Json is case sensitive - Match case
    map(literal("true"), |elem| Json::True { elem })
}

/// Parses a `false` terminal.
pub const fn false_<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // NOTE: Json is case sensitive - Match case
    map(literal("false"), |elem| Json::False { elem })
}

/// Parses a `null` terminal.
pub const fn null_<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // NOTE: Json is case sensitive - Match case
    map(literal("null"), |elem| Json::Null { elem })
}

/// Parses a `number` terminal.
pub const fn number<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    // NOTE: Json is case sensitive - Match case
    map(number_raw, |elem| Json::Number { elem })
}

/// Parses a `string` terminal.
pub const fn string<'a>() -> impl Parser<&'a str, &'a str, Json<'a>, String> {
    map(
        middle(literal("\""), string_content, literal("\"")),
        |elem| Json::String { elem },
    )
}

/// Parse a `number` terminal.
///
/// # Note
/// This is a concrete parser, it is an indirection to be able to use a non-const value in const
/// functions.
pub fn number_raw<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    // Note: Because it is not the point of the project, the regex used is a shameless steal from:
    //   https://stackoverflow.com/questions/13340717/json-numbers-regular-expression
    matching(&JSON_NUMBER_REGEX).parse(input)
}

/// Parse all json string char

/// # Note
/// This is a concrete parser, it is an indirection to be able to use a non-const value in const
/// functions.
pub fn string_content<'b, 'a: 'b>(input: &'a str) -> Result<(&'b str, &'b str), String> {
    // String gets a bit annoying as we may have escape character. A hand written parser
    // is better suited in this case.
    let mut chars = input.chars().peekable();
    let mut idx = 0;
    loop {
        match chars.peek() {
            // Consume 2 chars (escape plus next)
            Some('\\') => {
                chars.next();
                match chars.peek() {
                    Some(c) => chars.next(),
                    _ => return Err("Unexpected end of stream here".into()),
                };

                idx += 2;
            }

            // Probably hit the end of the string
            Some('"') => {
                break;
            }
            None => {
                break;
            }

            // Anything else, we consume
            _ => {
                chars.next();
                idx += 1;
            }
        }
    }

    if input.len() < idx {
        return Err("Derivation would exceed end of string".into());
    }

    Ok((&input[idx..], &input[0..idx]))
}

/// Example of usage of the terminal_value parser
#[test]
fn terminal_json_value_demo() {
    assert_eq!(
        Ok(("", Json::True { elem: "true" })),
        terminal_value().parse("true")
    );
    assert_eq!(
        Ok(("", Json::False { elem: "false" })),
        terminal_value().parse("false")
    );
    assert_eq!(
        Ok(("", Json::Null { elem: "null" })),
        terminal_value().parse("null")
    );

    assert_eq!(
        Ok(("", Json::Number { elem: "0.64" })),
        terminal_value().parse("0.64")
    );
    assert_eq!(
        Ok(("", Json::Number { elem: "-1" })),
        terminal_value().parse("-1")
    );
    assert_eq!(
        Ok(("", Json::Number { elem: "42" })),
        terminal_value().parse("42")
    );

    assert_eq!(
        Ok(("", Json::String { elem: "foo" })),
        terminal_value().parse("\"foo\"")
    );
    assert_eq!(
        Ok(("", Json::String { elem: "\\\\" })),
        terminal_value().parse("\"\\\\\"")
    );
    assert_eq!(
        Ok(("", Json::String { elem: "\\CODE" })),
        terminal_value().parse("\"\\CODE\"")
    );
    assert_eq!(
        Ok(("", Json::String { elem: "two words" })),
        terminal_value().parse("\"two words\"")
    );
}

#[test]
fn test_object() {}

#[test]
fn json_demo_1() {
    assert!(json("{}").is_ok());
    assert_eq!(Ok(Json::Object { elem: vec!() }), json("{}"));
}

#[test]
fn json_demo_2() {
    assert_eq!(
        Ok(Json::Object {
            elem: vec!((Json::String { elem: "foo" }, Json::String { elem: "bar" }))
        }),
        json(
            "{ \
        \"foo\": \"bar\" \
    }"
        )
    );
}

#[test]
fn json_demo_3() {
    assert_eq!(
        Ok(Json::Object {
            elem: vec!(
                (Json::String { elem: "foo" }, Json::String { elem: "bar" }),
                (
                    Json::String { elem: "2nd_key" },
                    Json::True { elem: "true" }
                )
            )
        }),
        json(
            "{ \
        \"foo\": \"bar\", \
        \"2nd_key\" : true \
    }"
        )
    );
}

#[test]
fn json_demo_4() {
    assert_eq!(
        Ok(Json::Array {
            elem: vec!(Json::String { elem: "bar" })
        }),
        json("[ \"bar\" ]")
    );
}

#[test]
fn json_demo_5() {
    assert_eq!(
        Ok(Json::Array {
            elem: vec!(
                Json::String { elem: "bar" },
                Json::String { elem: "foo" },
                Json::True { elem: "true" }
            )
        }),
        json("[\"bar\", \"foo\", true]")
    );
}

#[test]
fn json_demo_6() {
    assert_eq!(
        Ok(Json::Array {
            elem: vec!(
                Json::String { elem: "bar" },
                Json::String { elem: "foo" },
                Json::True { elem: "true" },
                Json::Object {
                    elem: vec!((Json::String { elem: "name" }, Json::String { elem: "bob" }))
                }
            )
        }),
        json("[\"bar\", \"foo\", true, {\"name\": \"bob\"}]")
    );
}

#[test]
fn json_demo_7() {
    assert_eq!(
        Ok(Json::Object {
            elem: vec!((
                Json::String { elem: "value" },
                Json::Array {
                    elem: vec!(
                        Json::String { elem: "bar" },
                        Json::String { elem: "foo" },
                        Json::True { elem: "true" },
                        Json::Object {
                            elem: vec!((
                                Json::String { elem: "name" },
                                Json::String { elem: "bob" }
                            ))
                        }
                    )
                }
            ))
        }),
        json(
            "{ \
    \"value\": [\
        \"bar\", \
        \"foo\", \
        true, \
        {\"name\": \"bob\"}\
    ]}"
        )
    );
}

#[test]
fn json_demo_sample() {
    assert!(json(include_str!("sample.json")).is_ok());
}
