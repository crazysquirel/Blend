use std::ops::Range;

use crate::{ parsers::SourceRange, parsers::ToRangeOption };

/// Ast representation of a Json in parsed form
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Json<'a> {
    /// Json Objects are simply mapping of Json values.
    ///
    /// # NOTE
    /// Using a vec allow to maintain order of insertion. Yields poorer lookups performance.
    Object {
        /// `object` representation
        elem: Vec<(Json<'a>, Json<'a>)>,
    },

    /// Array contains consecutive Json value without a key.
    Array {
        /// `array` representation
        elem: Vec<Json<'a>>,
    },

    /// `number` terminal
    Number {
        /// `number` representation
        elem: &'a str,
    },
    /// `string` terminal
    String {
        /// `string` representation
        elem: &'a str,
    },
    /// `true` terminal
    True {
        /// `true` representation
        elem: &'a str,
    },
    /// `false` terminal
    False {
        /// `false` representation
        elem: &'a str,
    },
    /// `null` terminal
    Null {
        /// `null` representation
        elem: &'a str,
    },
}

/// Showcase the implementation of the SourceRange trait.
///
/// # Note
/// Not used in the current crate.
impl<'a> SourceRange for Json<'a> {
    fn source_range(&self, source: &str) -> Option<Range<usize>> {
        match self {
            Self::Object { elem } => {
                let start = match elem.first() {
                    Some(e) => e.0.source_range_start(source),
                    None => None,
                };

                let end = match elem.last() {
                    Some(e) => e.0.source_range_end(source),
                    None => None,
                };
                (start..end).to_range()
            }
            Self::Array { elem } => {
                let start = match elem.first() {
                    Some(e) => e.source_range_start(source),
                    None => None,
                };

                let end = match elem.last() {
                    Some(e) => e.source_range_end(source),
                    None => None,
                };
                (start..end).to_range()
            }
            Self::Number { elem } => elem.source_range(source),
            Self::True { elem } => elem.source_range(source),
            Self::False { elem } => elem.source_range(source),
            Self::Null { elem } => elem.source_range(source),
            Self::String { elem } => elem.source_range(source),
        }
    }
}
