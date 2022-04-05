use std::ops::Range;

/// Given a source slice, attempts to retrieve a range corresponding to the slice which the
/// implementer represents.
///
/// NOTE: Returns None if the slice is not created from the source. A copy of the source will
///       also return None.
///
/// Design note: The idea is to unburden representations from indices. This is not the safest
///              way to proceed but it allows simpler, more concise code.
pub trait SourceRange {
    /// Given a valid source, retrieve the underlying slice that the implementer represents.
    /// Otherwise, return None
    ///
    /// # Example
    /// ```
    /// use parser_combinator::parsers::*;
    ///
    /// let source = "Foo is a dog";
    /// let parser = while_(left(identifier, maybe(whitespace)));
    /// let (_, vec) = parser.parse(source).expect("Parse is ok");
    /// assert_eq!(Some(4..6), vec[1].source_range(source));
    /// ```
    fn source_range(&self, source: &str) -> Option<Range<usize>>;

    /// Retrieve the start of the range, if the range is valid
    ///
    /// # Example
    /// ```
    /// use parser_combinator::parsers::*;
    ///
    /// let source = "Foo is a dog";
    /// let parser = while_(left(identifier, maybe(whitespace)));
    /// let (_, vec) = parser.parse(source).expect("Parse is ok");
    /// assert_eq!(Some(4), vec[1].source_range_start(source));
    /// ```
    fn source_range_start(&self, source: &str) -> Option<usize> {
        match self.source_range(source) {
            Some(r) => Some(r.start),
            None => None,
        }
    }

    /// Retrieve the end of the range, if the range is valid
    ///
    /// # Example
    /// ```
    /// use parser_combinator::parsers::*;
    ///
    /// let source = "Foo is a dog";
    /// let parser = while_(left(identifier, maybe(whitespace)));
    /// let (_, vec) = parser.parse(source).expect("Parse is ok");
    /// assert_eq!(Some(6), vec[1].source_range_end(source));
    /// ```
    fn source_range_end(&self, source: &str) -> Option<usize> {
        match self.source_range(source) {
            Some(r) => Some(r.end),
            None => None,
        }
    }
}

/// Implementation of SourceRange for slices. Allow easier manipulation from implementers of the
/// trait for most cases.
impl<'a> SourceRange for &'a str {
    fn source_range(&self, source: &str) -> Option<Range<usize>> {
        let start = self.as_ptr() as usize - source.as_ptr() as usize;
        let end = start + self.as_bytes().len();

        let start_is_oob = self.as_ptr() < source.as_ptr();
        let end_is_oob = (self.as_ptr() as usize + self.as_bytes().len())
            > (source.as_ptr() as usize + source.as_bytes().len());

        if start_is_oob || end_is_oob {
            return None;
        } else {
            Some(std::ops::Range { start, end })
        }
    }
}

/// Trait to convert any value to maybe a Range.
pub trait ToRangeOption<T> {
    /// Convert the implementer into and `Option<Range<T>>`
    ///
    /// # Example
    /// ```
    /// use parser_combinator::parsers::*;
    ///
    /// assert_eq!(Some(1..7), (Some(1)..Some(7)).to_range());
    /// ```
    fn to_range(self) -> Option<Range<T>>;
}

/// Blanket implementation of the ToRangeOption trait for ranges of Option<T> so that it does
/// not need to be implemented by all users of the trait.
///
/// # Design Note:
/// Allow for simpler manipulation of the SourceRange trait.
impl<T> ToRangeOption<T> for Range<Option<T>> {
    fn to_range(self) -> Option<Range<T>> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(Range { start, end }),
            _ => None,
        }
    }
}
