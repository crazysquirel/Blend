mod base_parsers;
pub use base_parsers::*;

mod source_range;
pub use source_range::*;


/// Main parser trait, pivotal to the library.
///
/// A parser is anything that can take an Input of type I and return a result
/// containing either a tuple with a Remainder of type R and an Output of type O OR
/// an error of type E.
///
/// Combinator functions contained in the module takes parsers which types correspond
/// and create a new parser from it.
pub trait Parser<I, R, O, E> {
    /// Should consume the next bit of input and returns either
    /// the remainder of the input and the desired object OR some kind of error.
    fn parse(&self, input: I) -> Result<(R, O), E>;
}

/// Auto implementation of the Parser trait for valid functions/closure.
impl<F, I, R, O, E> Parser<I, R, O, E> for F
where
    F: Fn(I) -> Result<(R, O), E>,
{
    fn parse(&self, input: I) -> Result<(R, O), E> {
        self(input)
    }
}

/// Parser that always succeed given any input.
///
/// # Result Conditions
/// Always succeeds.
pub const fn nothing<I, E>(input: I) -> Result<(I, ()), E> {
    Ok((input, ()))
}

/// Parser that always fail given any input.
///
/// # Result Conditions
/// Never succeeds.
pub const fn fail<P, I, R, O, E>(error: E) -> impl Parser<I, R, O, E>
where
    E: Clone,
{
    move |input: I| Err(error.clone())
}

/// Allow the transformation of a parser's output into another output.
///
/// # Result Conditions
/// Always succeeds.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = map(identifier, |i| ());
/// assert_eq!(Ok(("", ())), parser.parse("foo_1"));
/// ```
pub const fn map<P, I, R, OA, OB, E, FN>(pa: P, map_fn: FN) -> impl Parser<I, R, OB, E>
where
    P: Parser<I, R, OA, E>,
    FN: Fn(OA) -> OB,
{
    move |input: I| {
        let (remainder, ret) = pa.parse(input)?;
        Ok((remainder, map_fn(ret)))
    }
}

/// Takes two parsers and return the result of both in a tuple.
///
/// # Result Conditions
/// If either parser fails, the combined parser also fails.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = and(identifier, whitespace);
/// assert_eq!(Ok(("", ("ident", " "))), parser.parse("ident "));
/// ```
pub const fn and<PA, IA, RA, OA, PB, RB, OB, E>(pa: PA, pb: PB) -> impl Parser<IA, RB, (OA, OB), E>
where
    PA: Parser<IA, RA, OA, E>,
    PB: Parser<RA, RB, OB, E>,
{
    move |input: IA| {
        let (remainder, ret_a) = pa.parse(input)?;
        let (remainder, ret_b) = pb.parse(remainder)?;
        Ok((remainder, (ret_a, ret_b)))
    }
}

/// Takes two parsers and returns which ever result matches first.
///
/// Tries the first parser and then the second.
///
/// # Result Conditions
/// If both parser fails, the combined parser also fails. Otherwise succeeds.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = or(identifier, literal("--"));
/// assert_eq!(Ok(("", "ident")), parser.parse("ident"));
/// assert_eq!(Ok(("", "--")), parser.parse("--"));
/// ```
///
/// # Note
/// Try to have the least expensive input type as it will get cloned. For instance &'str is
/// inexpensive as it is just a pointer. Types that implement "Copy" are ideal but making
/// this a hard requirement would be too restrictive.
pub const fn or<PA, PB, I, R, O, E>(pa: PA, pb: PB) -> impl Parser<I, R, O, E>
where
    PA: Parser<I, R, O, E>,
    PB: Parser<I, R, O, E>,
    I: Clone,
{
    move |input: I| match pa.parse(input.clone()) {
        Ok(r) => Ok(r),
        Err(err) => match pb.parse(input) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        },
    }
}

/// Takes 2 parsers as argument and return the result of the second parser.
///
/// # Result Conditions
/// Both parser must succeed.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = right(identifier, whitespace);
/// assert_eq!(Ok(("", " ")), parser.parse("ident "));
/// ```
pub const fn right<PA, IA, RA, OA, PB, RB, OB, E>(pa: PA, pb: PB) -> impl Parser<IA, RB, OB, E>
where
    PA: Parser<IA, RA, OA, E>,
    PB: Parser<RA, RB, OB, E>,
{
    map(and(pa, pb), |(_, b)| b)
}

/// Takes 2 parsers as argument and return the result of the left parser. Both parser must succeed.
///
/// # Result Conditions
/// Both parser must succeed.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = left(identifier, whitespace);
/// assert_eq!(Ok(("", "ident")), parser.parse("ident "));
/// ```
pub const fn left<PA, IA, RA, OA, PB, RB, OB, E>(pa: PA, pb: PB) -> impl Parser<IA, RB, OA, E>
where
    PA: Parser<IA, RA, OA, E>,
    PB: Parser<RA, RB, OB, E>,
{
    map(and(pa, pb), |(a, _)| a)
}

/// Combination of the right, middle and left parser.
///
/// Combination of the right and left parser such that given parser A B C it returns the result
/// of B and the output of C or an exception of type E given an input valid for A.
///
/// # Result Conditions
/// All three parsers must succeed.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = middle(whitespace, identifier, whitespace);
/// assert_eq!(Ok(("", "ident")), parser.parse("\n ident \r"));
/// ```
pub const fn middle<PA, IA, RA, OA, PB, RB, OB, PC, RC, OC, E>(
    pa: PA,
    pb: PB,
    pc: PC,
) -> impl Parser<IA, RC, OB, E>
where
    PA: Parser<IA, RA, OA, E>,
    PB: Parser<RA, RB, OB, E>,
    PC: Parser<RB, RC, OC, E>,
{
    let p = map(and(pa, pb), |(_, b)| b);
    map(and(p, pc), |(m, _)| m)
}

/// Applies a parser 0 or more time. Always succeeds.
///
/// # Result Conditions
/// Always succeeds. Returns as soon as the given parser fails.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = while_(left(identifier, whitespace));
/// assert_eq!(Ok(("", vec!("ident1", "ident2"))), parser.parse("ident1 ident2 "));
/// assert_eq!(Ok(("", vec!())), parser.parse(""));
/// ```
pub const fn while_<P, I, O, E>(parser: P) -> impl Parser<I, I, Vec<O>, E>
where
    P: Parser<I, I, O, E>,
    I: Clone,
{
    move |input: I| {
        let mut rem = input;
        let mut res = Vec::new();

        loop {
            match parser.parse(rem.clone()) {
                Ok((new_rem, out)) => {
                    rem = new_rem;
                    res.push(out)
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok((rem, res))
    }
}

/// Applies a parser 1 or more time. Stops when the parser fails.
///
/// # Result Conditions
/// Succeed if the first parse succeeds and returns as soon as the given parser fails afterward.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = one_or_more(left(identifier, whitespace));
/// assert_eq!(Ok(("", vec!("ident1", "ident2"))), parser.parse("ident1 ident2 "));
/// assert!(parser.parse("").is_err());
/// ```
pub const fn one_or_more<P, I, O, E>(parser: P) -> impl Parser<I, I, Vec<O>, E>
where
    P: Parser<I, I, O, E>,
    I: Clone,
{
    move |input: I| {
        let mut rem = input;
        let mut res = Vec::new();

        let (first_rem, first_out) = parser.parse(rem.clone())?;
        res.push(first_out);
        rem = first_rem;

        loop {
            match parser.parse(rem.clone()) {
                Ok((new_rem, out)) => {
                    rem = new_rem;
                    res.push(out)
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok((rem, res))
    }
}

/// Applies a parser 0 or 1 time.
///
/// # Result Conditions
/// Always succeed but returns an option instead of a concrete value.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = left(identifier, maybe(whitespace));
/// assert_eq!(Ok(("", "ident1")), parser.parse("ident1 "));
/// assert_eq!(Ok(("", "ident1")), parser.parse("ident1"));
/// assert!(parser.parse("").is_err());
/// ```
pub const fn maybe<P, I, O, E>(parser: P) -> impl Parser<I, I, Option<O>, E>
where
    P: Parser<I, I, O, E>,
    I: Clone,
{
    move |input: I| match parser.parse(input.clone()) {
        Ok((rem, res)) => Ok((rem, Some(res))),
        Err(_) => Ok((input, None)),
    }
}

/// Applies the parser if the predicate is true. Always Succeeds.
///
/// # Result Conditions
/// Applies the parser, only if the predicates succeed, otherwise, succeed without parsing anything.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let predicate = |i: &str| i.chars().next().unwrap().is_alphabetic();
/// let parser = parse_if(identifier, predicate);
/// assert_eq!(Ok(("", Some("ident1"))), parser.parse("ident1"));
/// assert_eq!(Ok(("1ident", None)), parser.parse("1ident"));
/// ```
///
/// # Note
/// The predicate receives the input before it is parsed as a parameter.
pub const fn parse_if<P, I, O, E, FN>(parser: P, pred: FN) -> impl Parser<I, I, Option<O>, E>
where
    FN: Fn(I) -> bool,
    P: Parser<I, I, O, E>,
    I: Clone,
{
    move |input: I| {
        if pred(input.clone()) {
            parser
                .parse(input.clone())
                .map(|(rem, res)| (rem, Some(res)))
        } else {
            Ok((input, None))
        }
    }
}

/// Applies the parser if the predicate is true or returns an error.
///
/// # Result Conditions
/// Applies the parser, only if the predicates succeed, otherwise, fails and return the given error.
/// The predicate receives the input before it is parsed as a parameter.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let predicate = |i: &str| i.chars().next().unwrap().is_alphabetic();
/// let parser = parse_if_or_err(identifier, "Not valid".to_string(), predicate);
/// assert_eq!(Ok(("", "ident1")), parser.parse("ident1"));
/// assert_eq!(Err("Not valid".to_string()), parser.parse("1ident"));
/// ```
pub const fn parse_if_or_err<P, I, O, E, FN, T>(
    parser: P,
    error: T,
    pred: FN,
) -> impl Parser<I, I, O, E>
where
    FN: Fn(I) -> bool,
    P: Parser<I, I, O, E>,
    I: Clone,
    E: From<T>,
    T: Clone,
{
    move |input: I| {
        if pred(input.clone()) {
            parser.parse(input.clone())
        } else {
            Err(E::from(error.clone()))
        }
    }
}

/// Used for debugging, will call the given closure before applying the parser.
///
/// # Result Condition
/// Same as input parser.
///
/// # Example
/// ```
/// use parser_combinator::parsers::*;
///
/// let parser = inject(identifier, |i| println!("{}", i));
/// ```
///
/// # Note
/// By design, this is slower than using the parser directly for the exact same result.
/// Leaves the logic of the given parser untouched.
pub const fn inject<P, I, O, E, FN>(parser: P, func: FN) -> impl Parser<I, I, O, E>
where
    FN: Fn(I),
    P: Parser<I, I, O, E>,
    I: Clone,
{
    move |input: I| {
        func(input.clone());
        parser.parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::base_parsers::*;
    use super::*;

    #[test]
    fn test_and_parser() {
        let parser = and(identifier, whitespace);
        assert_eq!(Ok(("World", ("Hello", " "))), parser.parse("Hello World"));
    }

    #[test]
    fn test_map_parser() {
        let parser = and(identifier, whitespace);
        let parser = map(parser, |(_, b)| b);
        assert_eq!(Ok(("World", " ")), parser.parse("Hello World"));
    }

    #[test]
    fn test_right_parser() {
        let parser = right(identifier, whitespace);
        assert_eq!(Ok(("World", " ")), parser.parse("Hello World"));
    }

    #[test]
    fn test_left_parser() {
        let parser = left(identifier, whitespace);
        assert_eq!(Ok(("World", "Hello")), parser.parse("Hello World"));
    }

    #[test]
    fn test_or_parser() {
        // First succeed
        let parser = or(identifier, whitespace);
        assert_eq!(Ok((" World", "Hello")), parser.parse("Hello World"));

        // Second succeed
        let parser = or(identifier, whitespace);
        assert_eq!(Ok(("Hello World", " ")), parser.parse(" Hello World"));
    }

    #[test]
    fn test_while_parser() {
        // First succeed
        let parser = left(identifier, whitespace);
        let parser = while_(parser);
        assert_eq!(
            Ok(("", vec!("Hello", "World"))),
            parser.parse("Hello World ")
        );

        assert_eq!(Ok(("12345", vec!())), parser.parse("12345"));
    }

    #[test]
    fn test_one_or_more_parser() {
        let parser = left(identifier, whitespace);
        let parser = one_or_more(parser);
        assert_eq!(
            Ok(("", vec!("Hello", "World"))),
            parser.parse("Hello World ")
        );

        assert_eq!(Ok(("World", vec!("Hello"))), parser.parse("Hello World"));

        assert_eq!(
            Err(String::from("Could not parse '\\A[a-zA-Z_][a-zA-Z0-9_]*'")),
            parser.parse("12345")
        );

        let nothing = map(nothing, |_| "");
        let parser = left(identifier, or(whitespace, nothing));
        let parser = one_or_more(parser);
        assert_eq!(
            Ok(("", vec!("Hello", "World"))),
            parser.parse("Hello World")
        );

        assert_eq!(
            Ok(("12345", vec!("Hello", "World"))),
            parser.parse("Hello World 12345")
        );
    }

    #[test]
    fn test_parse_if_parser() {
        let parser = left(identifier, whitespace);
        let parser = parse_if(parser, |i| i.starts_with("H"));
        assert_eq!(Ok(("World", Some("Hello"))), parser.parse("Hello World"));

        assert_eq!(Ok(("Bye World", None)), parser.parse("Bye World"));
    }
}
