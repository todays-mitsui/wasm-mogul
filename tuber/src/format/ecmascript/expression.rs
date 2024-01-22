use super::super::formed::Formed;
use super::super::tag::Tag;
use crate::expr::Expr;
use crate::expr::Path;
use std::collections::HashSet;

pub fn format(expr: &Expr, splits: HashSet<Path>) -> Formed {
    tokenize(expr, &Tag::new()).into()
}

enum Compact<'a, 'b> {
    Variable {
        label: &'a str,
        tag: &'b Tag,
    },
    Symbol {
        label: &'a str,
        tag: &'b Tag,
    },
    Apply {
        callee: Box<Compact<'a, 'b>>,
        args: Vec<Compact<'a, 'b>>,
        tag: &'b Tag,
    },
    Lambda {
        params: Vec<&'a str>,
        body: Box<Compact<'a, 'b>>,
        tag: &'b Tag,
    },
}

fn expr_to_compact<'a, 'b>(expr: &'a Expr, tag: &'b Tag) -> Compact<'a, 'b> {
    match expr {
        Expr::Apply { lhs, rhs } => {
            let (callee, args) = expr.unapply();

            Compact::Apply {
                callee: Box::new(expr_to_compact(callee, tag)),
                args: args
                    .into_iter()
                    .enumerate()
                    .map(|(index, expr)| expr_to_compact(expr, &tag.push(index + 1)))
                    .collect(),
                tag,
            }
        }

        Expr::Variable(label) => Compact::Variable {
            label: label.as_str(),
            tag: &tag.push(0),
        },

        Expr::Symbol(label) => Compact::Symbol {
            label: label.as_str(),
            tag: &tag.push(0),
        },

        Expr::Lambda { param, body } => {
            let param = param.as_str();
            let body = expr_to_compact(&**body, &Tag::new());
            match body {
                Compact::Lambda {
                    mut params,
                    body,
                    tag,
                } => {
                    params.push(param);
                    Compact::Lambda {
                        params,
                        body,
                        tag: &tag.push(0),
                    }
                }
                _ => Compact::Lambda {
                    params: vec![param],
                    body: Box::new(body),
                    tag: &tag.push(0),
                },
            }
        }
    }
}

fn reform(compact: &mut Compact<'_, '_>, split: &[Path]) {
    match compact {}
}
