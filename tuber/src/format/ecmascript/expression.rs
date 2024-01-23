use super::super::formed::Formed;
use super::super::tag::Tag;
use crate::expr::Expr;
use crate::expr::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

// pub fn format(expr: &Expr, splits: HashSet<Path>) -> Formed {
//     tokenize(expr, &Tag::new()).into()
// }

#[derive(Debug, PartialEq)]
enum Compact<'a> {
    Variable {
        label: &'a str,
        tag: Tag,
    },
    Symbol {
        label: &'a str,
        tag: Tag,
    },
    Apply {
        callee: Box<Compact<'a>>,
        args: Vec<Compact<'a>>,
        tag: Tag,
    },
    Lambda {
        params: Vec<&'a str>,
        body: Box<Compact<'a>>,
        tag: Tag,
    },
}

fn expr_to_compact<'a>(expr: &'a Expr, tag: &Tag) -> Compact<'a> {
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
                tag: tag.to_owned(),
            }
        }

        Expr::Variable(label) => Compact::Variable {
            label: label.as_str(),
            tag: tag.push(0),
        },

        Expr::Symbol(label) => Compact::Symbol {
            label: label.as_str(),
            tag: tag.push(0),
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
                        tag: tag.push(0),
                    }
                }
                _ => Compact::Lambda {
                    params: vec![param],
                    body: Box::new(body),
                    tag: tag.push(0),
                },
            }
        }
    }
}

fn reform<'a>(compact: Compact<'a>, split: &[Path]) -> Compact<'a> {
    match compact {
        Compact::Apply { callee, args, tag } => {
            let next_paths: HashMap<usize, Vec<Path>> = group(
                split
                    .iter()
                    .filter_map(|path| match path {
                        Path::Arg(index, next) => Some((*index, (**next).clone())),
                        Path::Callee(_) => None,
                    })
                    .collect::<Vec<(usize, Path)>>(),
            );

            let new_args = args
                .into_iter()
                .enumerate()
                .map(|(index, arg)| {
                    let paths = next_paths.get(&index);
                    if let Some(paths) = paths {
                        reform(arg, paths)
                    } else {
                        arg
                    }
                })
                .collect();

            let arities = split
                .iter()
                .filter_map(|path| match path {
                    Path::Arg(_, _) => None,
                    Path::Callee(arity) => Some(*arity),
                })
                .collect::<Vec<_>>();

            if arities.is_empty() {
                Compact::Apply {
                    callee,
                    args: new_args,
                    tag,
                }
            } else {
                split_args(
                    Compact::Apply {
                        callee,
                        args: new_args,
                        tag,
                    },
                    arities,
                )
            }
        }
        _ => compact,
    }
}

fn group<K, V>(pairs: Vec<(K, V)>) -> HashMap<K, Vec<V>>
where
    K: Copy + Eq + Hash,
{
    let mut map = HashMap::new();
    for (key, value) in pairs {
        map.entry(key).or_insert_with(Vec::new).push(value);
    }
    map
}

fn split_args<'a>(compact: Compact<'a>, arities: Vec<usize>) -> Compact<'a> {
    if let Compact::Apply {
        mut callee,
        args,
        tag,
    } = compact
    {
        let arities = prepare(arities);

        let splited_args = split(args, &arities);

        for args in splited_args {
            callee = Box::new(Compact::Apply {
                callee,
                args,
                tag: tag.clone(),
            });
        }

        *callee
    } else {
        compact
    }
}

fn prepare(indices: Vec<usize>) -> Vec<usize> {
    let mut indices = indices
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|index| *index != 0)
        .collect::<Vec<_>>();

    indices.sort();

    indices
}

fn split<T: std::fmt::Debug>(mut list: Vec<T>, indices: &[usize]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut last_index = 0;
    for index in indices {
        let index = *index - last_index;
        let rest = list.split_off(index);

        result.push(list);

        list = rest;
        last_index += index;
    }

    if !list.is_empty() {
        result.push(list);
    }

    result
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    #[test]
    fn test_reform() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");
        let empty_tag = Tag::new();
        let compact = expr_to_compact(&expr, &empty_tag);

        let new_compact = reform(compact, &vec![Path::Callee(1), Path::Callee(3)]);

        println!("{:#?}", new_compact);

        assert_eq!(
            new_compact,
            Compact::Apply {
                callee: Box::new(Compact::Apply {
                    callee: Box::new(Compact::Apply {
                        callee: Box::new(Compact::Variable {
                            label: "f",
                            tag: Tag::from(vec![0])
                        }),
                        args: vec![Compact::Variable {
                            label: "w",
                            tag: Tag::from(vec![1, 0])
                        },],
                        tag: Tag::new()
                    }),
                    args: vec![
                        Compact::Variable {
                            label: "x",
                            tag: Tag::from(vec![2, 0])
                        },
                        Compact::Variable {
                            label: "y",
                            tag: Tag::from(vec![3, 0])
                        },
                    ],
                    tag: Tag::new()
                }),
                args: vec![Compact::Variable {
                    label: "z",
                    tag: Tag::from(vec![4, 0])
                },],
                tag: Tag::new()
            }
        );
    }

    #[test]
    fn test_split_args() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");
        let empty_tag = Tag::new();
        let compact = expr_to_compact(&expr, &empty_tag);

        let new_compact = split_args(compact, vec![1, 3]);

        println!("{:#?}", new_compact);

        assert_eq!(
            new_compact,
            Compact::Apply {
                callee: Box::new(Compact::Apply {
                    callee: Box::new(Compact::Apply {
                        callee: Box::new(Compact::Variable {
                            label: "f",
                            tag: Tag::from(vec![0])
                        }),
                        args: vec![Compact::Variable {
                            label: "w",
                            tag: Tag::from(vec![1, 0])
                        },],
                        tag: Tag::new()
                    }),
                    args: vec![
                        Compact::Variable {
                            label: "x",
                            tag: Tag::from(vec![2, 0])
                        },
                        Compact::Variable {
                            label: "y",
                            tag: Tag::from(vec![3, 0])
                        },
                    ],
                    tag: Tag::new()
                }),
                args: vec![Compact::Variable {
                    label: "z",
                    tag: Tag::from(vec![4, 0])
                },],
                tag: Tag::new()
            }
        );
    }

    #[test]
    fn test_split_1() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let indices = vec![2, 5, 7];

        let result = split(list, &indices);

        assert_eq!(
            result,
            vec![vec![1, 2], vec![3, 4, 5], vec![6, 7], vec![8],]
        );
    }

    #[test]
    fn test_split_2() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];
        let indices = vec![2, 5, 7];

        let result = split(list, &indices);

        assert_eq!(result, vec![vec![1, 2], vec![3, 4, 5], vec![6, 7]]);
    }
}
