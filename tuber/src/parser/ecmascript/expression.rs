use super::super::identifier::identifier;
use super::super::utils::{parens, token};
use crate::expr::{self, Expr, Identifier};
use combine::parser::char::{char, spaces, string};
use combine::parser::choice::choice;
use combine::{attempt, many, many1, optional, parser, ParseError, Parser, Stream};

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
            attempt(apply()),
            attempt(lambda()),
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

// ========================================================================== //

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
            .with(callable())
            .and(
                many1(spaces().with(args()))
            )
            .map(|(mut e, argss)| {
                let _: Vec<Vec<Expr>> = argss;
                for args in argss {
                    for arg in args {
                        e = expr::a(e, arg);
                    }
                }
                e
            })
    }
}

fn callable<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    callable_()
}

parser! {
    fn callable_[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces().with(choice((
            attempt(parens(expr())), // パーレンで囲まれている式はパーレンを剥がしてから再度パースを試みる

            // パーレンで囲まれていない場合、その後に許されるのは関数抽象, 変数, シンボルのみ
            // つまり、パーレンで囲まれていない形での関数適用は弾く
            // ここで関数適用を弾いておかないと左再帰で無限ループしてしまう
            attempt(lambda()),
            symbol(),
            var(),
        )))
    }
}

fn args<Input>() -> impl Parser<Input, Output = Vec<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    spaces()
        .with(parens(
            optional(many(attempt(token(expr()).skip(char(','))))).and(token(expr())),
        ))
        .map(|(es, e)| {
            let mut es: Vec<Expr> = es.unwrap_or_default();
            es.push(e);
            es
        })
}

// ========================================================================== //

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
            .with(
                choice((
                    params(),
                    identifier().map(|i| vec![i])),
                ))
                .skip(token(string("=>"))
            )
            .and(expr())
            .map(|(params, mut body)| {
                for param in params.into_iter().rev() {
                    body = expr::l(param, body);
                }
                body
            })
    }
}

fn params<Input>() -> impl Parser<Input, Output = Vec<Identifier>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    parens(optional(many(attempt(token(identifier()).skip(char(','))))).and(token(identifier())))
        .map(|(is, i)| {
            let mut is: Vec<Identifier> = is.unwrap_or_default();
            is.push(i);
            is
        })
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_expr() {
        assert_eq!(expr().easy_parse("a"), Ok((expr::v("a"), "")));
        assert_eq!(expr().easy_parse("a(b)"), Ok((expr::a("a", "b"), "")));
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
        assert_eq!(apply().easy_parse("a(b)"), Ok((expr::a("a", "b"), "")));
        assert_eq!(apply().easy_parse("(a)(b)"), Ok((expr::a("a", "b"), "")));
        assert_eq!(
            apply().easy_parse("a(b)(c)"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            apply().easy_parse("(a(b))(c)"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            apply().easy_parse(" a (  b   )"),
            Ok((expr::a("a", "b"), ""))
        );
        assert_eq!(
            apply().easy_parse("a(b, c)"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            apply().easy_parse(" a ( b ,  c  )"),
            Ok((expr::a(expr::a("a", "b"), "c"), ""))
        );
        assert_eq!(
            apply().easy_parse("FOO(BAR)"),
            Ok((expr::a("FOO", "BAR"), ""))
        );
        assert_eq!(apply().easy_parse(":a(b)"), Ok((expr::a(":a", "b"), "")));
        assert_eq!(
            apply().easy_parse("(x => x)(a)"),
            Ok((expr::a(expr::l("x", "x"), "a"), ""))
        );
    }

    #[test]
    fn test_lambda() {
        assert_eq!(lambda().easy_parse("a=>b"), Ok((expr::l("a", "b"), "")));
        assert_eq!(
            lambda().easy_parse(" a   =>  b"),
            Ok((expr::l("a", "b"), ""))
        );
        assert_eq!(
            lambda().easy_parse("a => b => c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
        assert_eq!(
            lambda().easy_parse("(a, b) => c"),
            Ok((expr::l("a", expr::l("b", "c")), ""))
        );
    }
}
