use super::tag::Tag;
use crate::expr::Expr;
use crate::expr::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq)]
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

// ========================================================================== //

impl<'a> From<&'a Expr> for Compact<'a> {
    fn from(expr: &'a Expr) -> Self {
        from_expr(expr, &Tag::new())
    }
}

fn from_expr<'a>(expr: &'a Expr, tag: &Tag) -> Compact<'a> {
    match expr {
        Expr::Apply { .. } => {
            let (callee, args) = expr.unapply();

            Compact::Apply {
                callee: Box::new(from_expr(callee, tag)),
                tag: tag.push(args.len()),
                args: args
                    .into_iter()
                    .enumerate()
                    .map(|(index, expr)| from_expr(expr, &tag.push(index + 1)))
                    .collect(),
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
            let body = from_expr(&**body, &Tag::new());
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

// ========================================================================== //

impl<'a> Compact<'a> {
    pub fn reform(self, split: &[Path]) -> Compact<'a> {
        if let Compact::Apply { callee, args, tag } = self {
            let next: HashMap<usize, Vec<Path>> = group(
                split
                    .iter()
                    .filter_map(|path| match path {
                        Path::Arg(index, next) => Some((*index, (**next).clone())),
                        Path::Callee(_) => None,
                    })
                    .collect::<Vec<(usize, Path)>>(),
            );

            let reformed_args: Vec<_> = args
                .into_iter()
                .enumerate()
                .map(|(index, arg)| {
                    let paths = next.get(&index);
                    if let Some(paths) = paths {
                        println!("{}: {:?}", index, paths);
                        println!("{:?}", arg);
                        arg.reform(paths)
                    } else {
                        arg
                    }
                })
                .collect();

            let arities: Vec<_> = split
                .iter()
                .filter_map(|path| match path {
                    Path::Arg(_, _) => None,
                    Path::Callee(arity) => Some(*arity),
                })
                .collect();

            if arities.is_empty() {
                Compact::Apply {
                    callee,
                    args: reformed_args,
                    tag: tag,
                }
            } else {
                Compact::Apply {
                    callee,
                    args: reformed_args,
                    tag,
                }
                .partition(arities)
            }
        } else {
            self
        }
    }

    fn partition(self, indices: Vec<usize>) -> Compact<'a> {
        if let Compact::Apply {
            mut callee,
            args,
            tag,
        } = self
        {
            let indices = prepare(indices);
            let splited_args = split(args, &indices);

            let mut arity = 0;
            for args in splited_args {
                arity += args.len();
                callee = Box::new(Compact::Apply {
                    callee,
                    args,
                    tag: tag.replace_last(arity),
                });
            }

            *callee
        } else {
            self
        }
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
    fn test_from() {
        let expr = expr::a(
            expr::a(
                expr::a(expr::a("a", "b"), "c"),
                expr::a(expr::a("d", "e"), "f"),
            ),
            expr::a("g", "h"),
        );

        let compact = Compact::from(&expr);

        assert_eq!(
            compact,
            Compact::Apply {
                callee: Box::new(Compact::Variable {
                    label: "a",
                    tag: Tag::from(vec![0])
                }),
                args: vec![
                    Compact::Variable {
                        label: "b",
                        tag: Tag::from(vec![1, 0])
                    },
                    Compact::Variable {
                        label: "c",
                        tag: Tag::from(vec![2, 0])
                    },
                    Compact::Apply {
                        callee: Box::new(Compact::Variable {
                            label: "d",
                            tag: Tag::from(vec![3, 0])
                        }),
                        args: vec![
                            Compact::Variable {
                                label: "e",
                                tag: Tag::from(vec![3, 1, 0])
                            },
                            Compact::Variable {
                                label: "f",
                                tag: Tag::from(vec![3, 2, 0])
                            },
                        ],
                        tag: Tag::from(vec![3, 2]),
                    },
                    Compact::Apply {
                        callee: Box::new(Compact::Variable {
                            label: "g",
                            tag: Tag::from(vec![4, 0])
                        }),
                        args: vec![Compact::Variable {
                            label: "h",
                            tag: Tag::from(vec![4, 1, 0])
                        },],
                        tag: Tag::from(vec![4, 1]),
                    }
                ],
                tag: Tag::from(vec![4]),
            }
        );
    }

    #[test]
    fn test_reform_1() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");
        let compact = Compact::from(&expr);

        let new_compact = compact.clone().reform(&vec![]);

        println!("{:#?}", new_compact);

        assert_eq!(new_compact, compact);
    }

    #[test]
    fn test_reform_2() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");
        let compact = Compact::from(&expr);

        let new_compact = compact.reform(&vec![Path::Callee(1), Path::Callee(3)]);

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
                        tag: Tag::from(vec![1]),
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
                    tag: Tag::from(vec![3]),
                }),
                args: vec![Compact::Variable {
                    label: "z",
                    tag: Tag::from(vec![4, 0])
                },],
                tag: Tag::from(vec![4]),
            }
        );
    }

    #[test]
    fn test_reform_3() {
        let expr = expr::a(
            expr::a(
                expr::a(expr::a("a", "b"), "c"),
                expr::a(expr::a("d", "e"), "f"),
            ),
            expr::a("g", "h"),
        );
        let compact = Compact::from(&expr);

        let new_compact = compact.reform(&vec![Path::Arg(2, Box::new(Path::Callee(1)))]);

        println!("{:#?}", new_compact);

        assert_eq!(
            new_compact,
            Compact::Apply {
                callee: Box::new(Compact::Variable {
                    label: "a",
                    tag: Tag::from(vec![0])
                }),
                args: vec![
                    Compact::Variable {
                        label: "b",
                        tag: Tag::from(vec![1, 0])
                    },
                    Compact::Variable {
                        label: "c",
                        tag: Tag::from(vec![2, 0])
                    },
                    Compact::Apply {
                        callee: Box::new(Compact::Apply {
                            callee: Box::new(Compact::Variable {
                                label: "d",
                                tag: Tag::from(vec![3, 0])
                            }),
                            args: vec![Compact::Variable {
                                label: "e",
                                tag: Tag::from(vec![3, 1, 0])
                            }],
                            tag: Tag::from(vec![3, 1]),
                        }),
                        args: vec![Compact::Variable {
                            label: "f",
                            tag: Tag::from(vec![3, 2, 0])
                        }],
                        tag: Tag::from(vec![3, 2]),
                    },
                    Compact::Apply {
                        callee: Box::new(Compact::Variable {
                            label: "g",
                            tag: Tag::from(vec![4, 0])
                        }),
                        args: vec![Compact::Variable {
                            label: "h",
                            tag: Tag::from(vec![4, 1, 0])
                        },],
                        tag: Tag::from(vec![4, 1]),
                    }
                ],
                tag: Tag::from(vec![4]),
            }
        );
    }

    #[test]
    fn test_partition_1() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");
        let compact = Compact::from(&expr);

        let new_compact = compact.partition(vec![1, 3]);

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
                        tag: Tag::from(vec![1]),
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
                    tag: Tag::from(vec![3]),
                }),
                args: vec![Compact::Variable {
                    label: "z",
                    tag: Tag::from(vec![4, 0])
                },],
                tag: Tag::from(vec![4]),
            }
        );
    }
}
