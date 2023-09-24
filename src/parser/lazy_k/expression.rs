use combine::parser::char::{char, spaces};
use combine::parser::choice::choice;
use combine::{parser, ParseError, Parser, Stream};

use super::super::identifier::identifier;
use crate::expr::{self, Expr};

pub fn expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    expr_()
}

parser! {
    fn expr_[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces().with(choice((
            apply(),
            lambda(),
            symbol(),
            var(),
        )))
    }
}

// ========================================================================== //

fn var<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    identifier().map(expr::v)
}

fn symbol<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char(':').with(identifier()).map(expr::s)
}

parser! {
    fn apply[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces()
            .with(char('`'))
            .with(expr())
            .and(expr())
            .map(|(lhs, rhs)| expr::a(lhs, rhs))
    }
}

parser! {
    fn lambda[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces()
            .with(choice((char('^'), char('λ'))))
            .with(
                identifier()
                .skip(spaces().with(char('.'))
            )
            .and(expr()))
            .map(|(param, body)| expr::l(param, body))
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_expr() {
        assert_eq!(expr().easy_parse("a"), Ok((expr::v("a"), "")));
        assert_eq!(expr().easy_parse("`ab"), Ok((expr::a("a", "b"), "")));
    }

    #[test]
    fn test_var() {
        assert!(var().easy_parse(":abc").is_err());
        assert!(var().easy_parse("^abc").is_err());

        assert_eq!(var().easy_parse("abc"), Ok((expr::v("a"), "bc")));
        assert_eq!(var().easy_parse("ABCabc"), Ok((expr::v("ABC"), "abc")));
    }

    #[test]
    fn test_symbol() {
        assert!(symbol().easy_parse("abc").is_err());

        assert_eq!(symbol().easy_parse(":abc"), Ok((expr::s("a"), "bc")));
        assert_eq!(symbol().easy_parse(":ABCabc"), Ok((expr::s("ABC"), "abc")));
    }

    #[test]
    fn test_apply() {
        assert!(expr().easy_parse("`a").is_err());

        assert_eq!(expr().easy_parse("`ab"), Ok((expr::a("a", "b"), "")));
        assert_eq!(expr().easy_parse(" ` a b"), Ok((expr::a("a", "b"), "")));
        assert_eq!(
            expr().easy_parse("``abc"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            expr().easy_parse(" ` ` a b c"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            expr().easy_parse("`FOO BAR"),
            Ok((expr::a("FOO", "BAR"), ""))
        );
    }

    #[test]
    fn test_lambda() {
        assert!(expr().easy_parse("^a").is_err());
        assert_eq!(expr().easy_parse("^a.b"), Ok((expr::l("a", "b"), "")));
        assert_eq!(expr().easy_parse(" ^ a . b"), Ok((expr::l("a", "b"), "")));

        assert!(expr().easy_parse("λa").is_err());
        assert_eq!(expr().easy_parse("λa.b"), Ok((expr::l("a", "b"), "")));
        assert_eq!(expr().easy_parse(" λ a . b"), Ok((expr::l("a", "b"), "")));

        assert_eq!(
            expr().easy_parse("^a.^b.c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
        assert_eq!(
            expr().easy_parse("λa.λb.c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
        assert_eq!(
            expr().easy_parse("λa.^b.c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );

        assert_eq!(
            expr().easy_parse(" ^ a . ^ b . c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
        assert_eq!(
            expr().easy_parse(" λ a . λ b . c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
        assert_eq!(
            expr().easy_parse(" ^ a . λ b . c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
    }
}
