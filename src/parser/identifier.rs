use combine::parser::char::{char, digit, lower, spaces, upper};
use combine::parser::choice::choice;
use combine::{many1, ParseError, Parser, Stream};

use crate::expr::Identifier;

pub fn identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(choice((short_identifier(), iota(), long_identifier())))
}

fn short_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    lower().map(|c| {
        let mut buffer = [0; 4];
        let s: &str = c.encode_utf8(&mut buffer);
        s.into()
    })
}

fn long_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(choice((digit(), upper(), char('_')))).map(|s: String| s.into())
}

fn iota<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char('ι').map(|c| "ι".into())
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier().easy_parse("abc"), Ok(("a".into(), "bc")));
        assert_eq!(identifier().easy_parse("ABC"), Ok(("ABC".into(), "")));
        assert_eq!(identifier().easy_parse("ABCabc"), Ok(("ABC".into(), "abc")));
        assert_eq!(identifier().easy_parse("A_B_C"), Ok(("A_B_C".into(), "")));
        assert_eq!(identifier().easy_parse("42"), Ok(("42".into(), "")));

        assert!(identifier().easy_parse(":abc").is_err());
        assert!(identifier().easy_parse("^abc").is_err());
    }

    #[test]
    fn test_short_identifier() {
        assert_eq!(short_identifier().easy_parse("a"), Ok(("a".into(), "")));

        assert!(short_identifier().easy_parse("A").is_err());
    }

    #[test]
    fn test_long_identifier() {
        assert!(long_identifier().easy_parse("abc").is_err());

        assert_eq!(long_identifier().easy_parse("ABC"), Ok(("ABC".into(), "")));
        assert_eq!(
            long_identifier().easy_parse("ABCabc"),
            Ok(("ABC".into(), "abc"))
        );
    }
}
