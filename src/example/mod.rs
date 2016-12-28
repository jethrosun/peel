//! Example parsers
mod parser1;
mod parser2;
mod parser3;
mod parser4;

use std::fmt;
use self::prelude::*;

pub mod prelude {
    //! Sensible defaults for the example parsers
    pub use super::*;
    pub use prelude::*;
    pub use nom::IResult;

    /// Shorthand for our own personal memory arena
    pub type ExampleArena = ParserArena<ParserResult, ParserVariant>;

    /// Shorthand for our own personal node
    pub type ExampleNode = ParserNode<ParserResult, ParserVariant>;

    pub use example::parser1::*;
    pub use example::parser2::*;
    pub use example::parser3::*;
    pub use example::parser4::*;
}

/// Collects all possible parser variants
pub enum ParserVariant {
    /// First example parser
    Variant1(Parser1),

    /// Second example parser
    Variant2(Parser2),

    /// Third example parser
    Variant3(Parser3),

    /// Fourth example parser
    Variant4(Parser4),
}

#[derive(PartialEq, Debug)]
/// Return values of the parsers
pub enum ParserResult {
    /// The result of the first example parser
    Result1(bool),

    /// The result of the second example parser
    Result2(bool),

    /// The result of the third example parser
    Result3(bool),

    /// The result of the fourth example parser
    Result4(bool),
}

/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<ParserResult, ParserVariant> {
    // Create a tree
    let mut p = Peel::new();

    // Create and link the parsers
    let parser_1 = p.new_parser(Parser1);

    // Append Parser2 to Parser1
    p.link_new_parser(parser_1, Parser2);

    // Append Parser3 to Parser1
    let parser_3 = p.link_new_parser(parser_1, Parser3);

    // Append Parser4 to Parser3
    p.link_new_parser(parser_3, Parser4);

    p
}

impl fmt::Display for ParserVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserVariant::Variant1(_) => write!(f, "Parser 1"),
            ParserVariant::Variant2(_) => write!(f, "Parser 2"),
            ParserVariant::Variant3(_) => write!(f, "Parser 3"),
            ParserVariant::Variant4(_) => write!(f, "Parser 4"),
        }
    }
}
