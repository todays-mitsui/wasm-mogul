use combine::parser::char::{char, digit, spaces, string};
use combine::parser::choice::choice;
use combine::{
    attempt, count_min_max, eof, many, many1, optional, parser, ParseError, Parser, Stream,
};

use super::super::identifier::identifier;
use super::expression::expr;
use crate::engine::Command;
use crate::expr::{Expr, Identifier};
use crate::func;

pub fn command<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    choice((
        attempt(update()),
        eval(),
        attempt(eval_head()),
        attempt(eval_tail()),
        eval_last(),
        attempt(unlambda()),
        attempt(search()),
        global(),
    ))
    .skip(spaces())
    .skip(eof())
}

// ========================================================================== //

pub fn update<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    def_lhs()
        .skip(spaces().with(char('=')))
        .and(expr())
        .map(|((i, is), rhs)| match rhs {
            Expr::Variable(j) if is.is_empty() && i == j => Command::Del(i),
            _ => Command::Update(func::new(i, is, rhs)),
        })
}

fn def_lhs<Input>() -> impl Parser<Input, Output = (Identifier, Vec<Identifier>)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    def_lhs_()
}

parser! {
    fn def_lhs_[Input]()(Input) -> (Identifier, Vec<Identifier>)
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        identifier()
            .and(spaces().with(optional(params())))
            .map(|(i, is)| (i, is.unwrap_or_else(Vec::new)))
    }
}

fn params<Input>() -> impl Parser<Input, Output = Vec<Identifier>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    char('(')
        .with(
            optional(many(attempt(
                spaces().with(identifier()).skip(spaces()).skip(char(',')),
            )))
            .and(spaces().with(identifier()))
            .map(|(is, i)| {
                let mut is = is.unwrap_or_else(Vec::new);
                is.push(i);
                is
            }),
        )
        .skip(spaces().with(char(')')))
}

// ========================================================================== //

fn eval<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    expr().map(Command::Eval)
}

fn eval_last<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    char('!').with(expr()).map(Command::EvalLast)
}

fn eval_head<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    let len = many1(digit()).and_then(|x: String| x.parse::<usize>());

    char('!')
        .with(len)
        .and(spaces().with(expr()))
        .map(|(len, e)| Command::EvalHead(len, e))
}

fn eval_tail<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    let len = many1(digit()).and_then(|x: String| x.parse::<usize>());

    string("!-")
        .with(len)
        .and(spaces().with(expr()))
        .map(|(len, e)| Command::EvalTail(len, e))
}

// ========================================================================== //

fn search<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces()
        .skip(char('?'))
        .with(identifier())
        .map(Command::Search)
}

fn global<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().skip(char('?')).map(|_| Command::Context)
}

// ========================================================================== //

fn unlambda<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    let level = count_min_max(1, 4, char('~')).map(|str: String| str.len() as u8);

    spaces()
        .with(level)
        .and(spaces().with(expr()))
        .map(|(level, expr)| Command::Unlambda(level, expr))
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Command;
    use crate::expr;
    use combine::EasyParser;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            command().easy_parse("f=g"),
            Ok((Command::Update(func::new("f", Vec::<&str>::new(), "g")), ""))
        );

        assert_eq!(
            command().easy_parse("i(x) = x"),
            Ok((Command::Update(func::new("i", vec!["x"], "x")), ""))
        );

        assert_eq!(
            command().easy_parse("s(x, y, z) = x(z, y(z))"),
            Ok((
                Command::Update(func::new(
                    "s",
                    vec!["x", "y", "z"],
                    expr::a(expr::a("x", "z"), expr::a("y", "z"))
                )),
                ""
            ))
        );

        assert_eq!(
            command().easy_parse("a(b)"),
            Ok((Command::Eval(expr::a("a", "b")), ""))
        );

        assert_eq!(
            command().easy_parse("? a"),
            Ok((Command::Search("a".into()), ""))
        );

        assert_eq!(command().easy_parse("?"), Ok((Command::Context, "")));

        assert!(command().easy_parse("f=g h=i").is_err());
    }

    #[test]
    fn test_command() {
        assert_eq!(
            command().easy_parse("f=g"),
            Ok((Command::Update(func::new("f", Vec::<&str>::new(), "g")), ""))
        );

        assert_eq!(
            command().easy_parse("i(x) = x"),
            Ok((Command::Update(func::new("i", vec!["x"], "x")), ""))
        );

        assert_eq!(
            command().easy_parse("s(x, y, z) = x(z, y(z))"),
            Ok((
                Command::Update(func::new(
                    "s",
                    vec!["x", "y", "z"],
                    expr::a(expr::a("x", "z"), expr::a("y", "z"))
                )),
                ""
            ))
        );

        assert_eq!(
            command().easy_parse("a(b)"),
            Ok((Command::Eval(expr::a("a", "b")), ""))
        );

        assert_eq!(
            command().easy_parse("!a(b)"),
            Ok((Command::EvalLast(expr::a("a", "b")), ""))
        );

        assert_eq!(
            command().easy_parse("!42 a(b)"),
            Ok((Command::EvalHead(42, expr::a("a", "b")), ""))
        );

        assert_eq!(
            command().easy_parse("!-42 a(b)"),
            Ok((Command::EvalTail(42, expr::a("a", "b")), ""))
        );

        assert_eq!(
            command().easy_parse("? a"),
            Ok((Command::Search("a".into()), ""))
        );

        assert_eq!(command().easy_parse("?"), Ok((Command::Context, "")));
    }

    #[test]
    fn test_def() {
        assert_eq!(
            update().easy_parse("f=g"),
            Ok((Command::Update(func::new("f", Vec::<&str>::new(), "g")), ""))
        );

        assert_eq!(
            update().easy_parse("f = g"),
            Ok((Command::Update(func::new("f", Vec::<&str>::new(), "g")), ""))
        );

        assert_eq!(
            update().easy_parse("i(x) = x"),
            Ok((Command::Update(func::new("i", vec!["x"], "x")), ""))
        );

        assert_eq!(
            update().easy_parse("s(x, y, z) = x(z, y(z))"),
            Ok((
                Command::Update(func::new(
                    "s",
                    vec!["x", "y", "z"],
                    expr::a(expr::a("x", "z"), expr::a("y", "z"))
                )),
                ""
            ))
        );
    }

    #[test]
    fn test_def_lhs() {
        assert_eq!(def_lhs().easy_parse("f"), Ok((("f".into(), vec![]), "")));

        assert_eq!(
            def_lhs().easy_parse("f(x)"),
            Ok((("f".into(), vec!["x".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("f (  x   )"),
            Ok((("f".into(), vec!["x".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("f(x, y)"),
            Ok((("f".into(), vec!["x".into(), "y".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("f  ( x   , y  )"),
            Ok((("f".into(), vec!["x".into(), "y".into()]), ""))
        );

        assert!(def_lhs().easy_parse("f(x(y))").is_err());
    }

    #[test]
    fn test_params() {
        assert_eq!(
            params().easy_parse("(x, y, z)"),
            Ok((vec!["x".into(), "y".into(), "z".into()], ""))
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval().easy_parse("a"), Ok((Command::Eval("a".into()), "")));
        assert_eq!(
            eval().easy_parse("a(b)"),
            Ok((Command::Eval(expr::a("a", "b")), ""))
        );
    }

    #[test]
    fn test_search() {
        assert_eq!(
            search().easy_parse("?a"),
            Ok((Command::Search("a".into()), ""))
        );
        assert_eq!(
            search().easy_parse("? a"),
            Ok((Command::Search("a".into()), ""))
        );
    }

    #[test]
    fn test_global() {
        assert_eq!(global().easy_parse("?"), Ok((Command::Context, "")));
    }

    #[test]
    fn test_unlambda() {
        assert_eq!(
            unlambda().easy_parse("~x=>x"),
            Ok((Command::Unlambda(1, expr::l("x", "x")), ""))
        );

        assert_eq!(
            unlambda().easy_parse("~~x=>x"),
            Ok((Command::Unlambda(2, expr::l("x", "x")), ""))
        );

        assert_eq!(
            unlambda().easy_parse("~~~x=>x"),
            Ok((Command::Unlambda(3, expr::l("x", "x")), ""))
        );

        assert_eq!(
            unlambda().easy_parse("~~~~x=>x"),
            Ok((Command::Unlambda(4, expr::l("x", "x")), ""))
        );

        assert!(unlambda().easy_parse("~~~~~x=>x").is_err());
    }
}
