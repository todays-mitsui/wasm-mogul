use combine::parser::char::{char, spaces};
use combine::{ParseError, Parser, Stream};

pub fn token<Input, Output>(
    parser: impl Parser<Input, Output = Output>,
) -> impl Parser<Input, Output = Output>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    spaces().with(parser).skip(spaces())
}

pub fn parens<Input, Output>(
    parser: impl Parser<Input, Output = Output>,
) -> impl Parser<Input, Output = Output>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    char('(')
        .with(spaces().with(parser))
        .skip(spaces().with(char(')')))
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_token() {
        assert_eq!(token(char('a')).easy_parse(" a "), Ok(('a', "")));
        assert_eq!(token(char('a')).easy_parse("a"), Ok(('a', "")));

        assert!(token(char('a')).easy_parse("b").is_err());
    }

    #[test]
    fn test_parens() {
        assert_eq!(parens(char('a')).easy_parse("(a)"), Ok(('a', "")));
        assert_eq!(parens(char('a')).easy_parse("( a )"), Ok(('a', "")));

        assert!(parens(char('a')).easy_parse("a").is_err());
        assert!(parens(char('a')).easy_parse("((a))").is_err());
    }
}
