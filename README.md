# ***Blend*** - A parser combinator library

## Context 
This small library act as a way to demonstrate a sample of my skill set in Rust. It is not meant for
usage as-is, nor is it mean to be exhaustive. It was first conceived while needing to upgrade the way 
some logs for distributed jobs were parsed. Unfortunately, these logs contained sensitive information 
so the parser for them and the test data associated with them had to be removed. In its place, an example 
Json parser has been added to the project as a way to showcase the library's use. 

## Points of interest
If you are unfamiliar with the parsing technique, I recommend to finish reading the `README` to get a
better idea of the content of the library. However, here is a quick suggestion of reading order for 
those interested in looking at the code straight away:
- Start with the [parser definition](src/parsers/mod.rs) and the associated combinators.
- Then check some [root examples](src/parsers/base_parsers.rs) which are the smallest unit parsers.
- Finally, jump in the [json example](src/json/parser.rs) which showcases how the library is meant to 
be used.

## Quick checks
`cargo test` will quickly verify the few tests and doc tests in the library. `cargo doc --open` will
open the documentation for the crate.

## What are parser combinators ?
> ***Note*** \
> This is meant as a 10 000 ft view of a parser design pattern. Some concepts may be fudged a little. 
> For more information [here is an excellent example in Haskell](https://hasura.io/blog/parser-combinators-walkthrough/) 
> and [here is another](https://bodil.lol/parser-combinators/) one in Rust not unlike the one implemented 
> here even though some type representation choices differ.

Parser combinators is a parser design pattern in which the aim is to create complete parsers from a 
series of small derivation (or "chunks of content") parsers that are glued together with higher order functions.
This is to say, the parser functions are given as input to another function which combines them and return
a new function. The new function itself is also parser.

The overarching goal is to be able to write code which match very closely any given
[parsing expression grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar).

### In practice

Concretely, this requires to abstract the concept of a parser. Since this is a functional approach
(as in the paradigm), it only makes sense to start by reasoning about the type. So what is a parser?
A parser is a `Function` that takes an `Input` and returns either a `Remainder` and a `Parsed value` 
or an `Error`. If we glue it together we get a signature along the line of:

```rust
fn some_parser<I, R, O, E>(input: I) -> Result<(R, O), E> { /* ... */}
```

The neat bit is that even if we don't know what the parser does, we can know if it succeeded, or 
it failed. This is where combination starts to make sense. If we know the result of a parser, we can
take decision whether we should continue the computation or not. However, to write a combinator, we 
must first define a bit more formally to the compiler what is a parser: 

```rust
pub trait Parser<I, R, O, E> {
    fn parse(&self, input: I) -> Result<(R, O), E>;
}
```

Notice that the `parse` function has the same signature as our earlier definition. This allows us 
to implement the trait automatically for all function that comport themselves as parser, like so
by simply letting the function call itself:

```rust
impl<F, I, R, O, E> Parser<I, R, O, E> for F
where
    F: Fn(I) -> Result<(R, O), E>,
{
    fn parse(&self, input: I) -> Result<(R, O), E> {
        self(input)
    }
}
```

With these building block we can then make something more sophisticated. Say we have a 
`Parser<AI, AR, AO, E>` and a `Parser<AR, BR, BO, E>`, we have what is called a compatible parser pair.
Specifically, we can se that the remainder of the first parser is the same type as the input for the 
second one, which means that on a success, its output can be fed to the second parser. Armed with this 
knowledge, here is how we could define a `and` parser that will succeed if and only if both parser succeed.
Its return type should be both Output since both parser must succeed. Here is what is meant:

```rust
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
```

Notice that instead of a single value, we now return a tuple! If either parser fails, the `and` parser 
itself fails. In [the parsers module](src/parsers/mod.rs) you will find more definitions as well as other
design pattern that can be mixed and matched such as `map` in order to make it easier to work with parser
combinators.


## Goals and Non-Goals
### Goals
#### Type system usage demonstration
The current library aims to showcase my abilities to reason with the rust type system and play with some 
of its generic capability.

#### Code organisation
One of the goal is to also demonstrate how code can be made to be as modular as possible. The favored 
approach in the project is more akin to the functional programing paradigm, but something more oop-like
could have been favored.

#### Ergonomics
A special care has been taken to improve the ergonomics of the library so that combination is as easy as
possible. There are some pitfalls here and there, but overall, it plays extremely well with type inference
and allow the users to cut down on the verbosity of the code.

### Non-Goals
#### Disrupt actual existing solutions
This is meant to showcase a subset of my abilities with the language, as such, it does not pretend to be
better than actual dedicated libraries that aspire to be adopted by a wide audience.

#### Showcase teamwork
This is more of a personal demonstration to prove that I can generally program my way out of a paper bag.
Think of it as a kind of [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz). As so, this project was 
specifically chosen because (not "even though") I worked alone. Teamwork could be demonstrated through other 
better means such as references and interviews would it be necessary.

#### Demonstrate error handling
While I could talk length (or a "fair" bit at least) about error handling, it was not one of the aim of this
project as the responsibility befalls the author of the concrete parser instead of the combinators. As such,
to keep the Json example brief, error handling was mostly waved out.

#### Demonstrate proper test implementation
Tests are non-exhaustive and more of a simple form of sanity check. Doing loads of unit tests, fuzzing tests
and integration tests is out of the scope of this project.

#### Demonstrate Object-Oriented Programing like practices in Rust
While Rust allows for writing project in a somewhat Oop like form, this was not the aim of the current project.
As such, not many struct are defined, neither do they implement a lot of traits. This could be demonstrated
in other projects if the need was to arise. 

#### Have crazy good performance
The library has a very decent performance, thanks to most of it being `const` and thus known at compile time. 
However, it will **never** beat a dedicated 
[handwritten json simd parser](https://github.com/simdjson/simdjson#performance-results) for example.
This kind of top shelf code gets quickly very verbose and complicated, which makes for a poor concise showcase.

## Conclusion
Thanks for taking an interest in my project, I hope this demonstrates what you were looking for!