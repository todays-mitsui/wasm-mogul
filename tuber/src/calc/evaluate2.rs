use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;
use std::{cmp, iter, slice};

#[derive(Debug, PartialEq)]
pub struct Eval {
    context: Context,
    inventory: Inventory,
}

impl Eval {
    pub fn new(context: Context, expr: Expr) -> Self {
        Self {
            inventory: Inventory::new(&context, expr),
            context,
        }
    }

    fn focus_path(&self) -> FocusPath {
        match self.inventory.focused() {
            Focused::Done => FocusPath::Done,
            Focused::Callee(_) => FocusPath::empty_path(),
            Focused::Arg { index, inventory } => {
                let mut path = FocusPath::empty_path();
                path.push(index);

                let mut inventory = inventory;
                while let Focused::Arg {
                    index,
                    inventory: inner,
                } = inventory.focused()
                {
                    path.push(index);
                    inventory = inner;
                }

                path
            }
        }
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
struct Inventory {
    focus: Focus,
    callee: Expr,
    args: Args,
}

impl Inventory {
    fn new(context: &Context, expr: Expr) -> Self {
        let mut callee = expr;
        let mut args = Args::new();

        while let Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.unshift(context, *rhs);
        }

        let callable: bool = arity(context, &callee)
            .map(|arity| args.len() >= cmp::max(1, arity))
            .unwrap_or_default();

        if callable {
            Self {
                focus: Focus::Callee,
                callee,
                args,
            }
        } else {
            for (index, arg) in args.iter::<(usize, &Inventory)>() {
                if arg.is_normal() {
                    continue;
                } else {
                    return Self {
                        focus: Focus::Arg(index),
                        callee,
                        args,
                    };
                }
            }

            Self {
                focus: Focus::Done,
                callee,
                args,
            }
        }
    }

    /// 正規形か否かを判定する
    fn is_normal(&self) -> bool {
        self.focus == Focus::Done
    }

    fn focused(&self) -> Focused {
        match self.focus {
            Focus::Callee => Focused::Callee(&self.callee),
            Focus::Arg(index) => Focused::Arg {
                index,
                inventory: self.arg(index).unwrap(),
            },
            Focus::Done => panic!("cannot focus done inventory"),
        }
    }

    fn arg(&self, index: usize) -> Option<&Inventory> {
        self.args.0.get(index)
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
enum Focus {
    Callee,
    Arg(usize),
    Done,
}

enum Focused<'a> {
    Callee(&'a Expr),
    Arg {
        index: usize,
        inventory: &'a Inventory,
    },
    Done,
}

#[derive(Debug, PartialEq)]
enum FocusPath {
    Path(Vec<usize>),
    Done,
}

impl FocusPath {
    fn empty_path() -> Self {
        Self::Path(Vec::new())
    }

    fn push(&mut self, index: usize) {
        match self {
            Self::Path(path) => path.push(index),
            Self::Done => panic!("cannot push to done path"),
        }
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
struct Args(Vec<Inventory>);

/// ラムダ式の部分式のうち引数部分を保持する両端キュー
/// 実装の都合で内部的には引数を逆順で保持する
/// ```sxyz を分解して格納した場合、外部的には [x, y, z] として振る舞い、内部的には [z, y, x] というデータを保持する
impl Args {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn unshift(&mut self, context: &Context, expr: Expr) {
        let inventory = Inventory::new(context, expr);
        self.0.push(inventory)
    }

    // fn iter(&self) -> impl Iterator<Item = (usize, &Inventory)> {
    // TODO: このように impl Trait の形で書くとうまくいかない
    // TODO: Error: cannot move out of `args` because it is borrowed
    fn iter<Iter>(&self) -> iter::Enumerate<iter::Rev<slice::Iter<'_, Inventory>>> {
        self.0.iter().rev().enumerate()
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    fn setup() -> Context {
        Context::default()
    }

    #[test]
    fn test_eval_new() {
        let context = setup();
        let expr = expr::v("x");
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                focus: Focus::Done,
                callee: expr::v("x"),
                args: Args::new(),
            }
        );

        // ================================================================== //

        let context = setup();
        let expr = expr::a("f", "x");
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                focus: Focus::Done,
                callee: expr::v("f"),
                args: Args(vec![Inventory {
                    focus: Focus::Done,
                    callee: expr::v("x"),
                    args: Args::new(),
                }]),
            }
        );

        // ================================================================== //

        let context = setup();
        let expr = expr::a("i", "x");
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                focus: Focus::Callee,
                callee: expr::v("i"),
                args: Args(vec![Inventory {
                    focus: Focus::Done,
                    callee: expr::v("x"),
                    args: Args::new(),
                }]),
            }
        );

        // ================================================================== //

        let context = setup();
        let expr = expr::a(":f", expr::a(expr::a("i", ":x"), expr::a("i", ":y")));
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                focus: Focus::Arg(0),
                callee: expr::s("f"),
                args: Args(vec![Inventory {
                    focus: Focus::Callee,
                    callee: expr::v("i"),
                    args: Args(vec![
                        Inventory {
                            focus: Focus::Callee,
                            callee: expr::v("i"),
                            args: Args(vec![Inventory {
                                focus: Focus::Done,
                                callee: expr::s("y"),
                                args: Args::new(),
                            }]),
                        },
                        Inventory {
                            focus: Focus::Done,
                            callee: expr::s("x"),
                            args: Args::new(),
                        }
                    ]),
                }]),
            }
        );

        // ================================================================== //

        let context = setup();
        let expr = expr::a(":f", expr::a(expr::a(":i", ":x"), expr::a("i", ":y")));
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                focus: Focus::Arg(0),
                callee: expr::s("f"),
                args: Args(vec![Inventory {
                    focus: Focus::Arg(1),
                    callee: expr::s("i"),
                    args: Args(vec![
                        Inventory {
                            focus: Focus::Callee,
                            callee: expr::v("i"),
                            args: Args(vec![Inventory {
                                focus: Focus::Done,
                                callee: expr::s("y"),
                                args: Args::new(),
                            }]),
                        },
                        Inventory {
                            focus: Focus::Done,
                            callee: expr::s("x"),
                            args: Args::new(),
                        }
                    ]),
                }]),
            }
        );
    }

    #[test]
    fn test_eval_focus_path() {
        let expr = expr::a(expr::a(expr::a("s", ":x"), ":y"), ":z");
        let path = FocusPath::Path(vec![]);
        let eval = Eval::new(Context::default(), expr);
        assert_eq!(eval.focus_path(), path);

        let expr = expr::a(":f", expr::a(expr::a("k", ":x"), ":y"));
        let path = FocusPath::Path(vec![0]);
        let eval = Eval::new(Context::default(), expr);
        assert_eq!(eval.focus_path(), path);

        let expr = expr::a(":f", expr::a(":g", expr::a("i", ":x")));
        let path = FocusPath::Path(vec![0, 0]);
        let eval = Eval::new(Context::default(), expr);
        assert_eq!(eval.focus_path(), path);

        let expr = expr::a(":f", expr::a(expr::a("i", ":x"), expr::a("i", ":y")));
        let path = FocusPath::Path(vec![0]);
        let eval = Eval::new(Context::default(), expr);
        assert_eq!(eval.focus_path(), path);

        // // TODO: cannot focus done inventory
        // let expr = expr::a(":f", expr::a(expr::a(":i", ":x"), expr::a("i", ":y")));
        // let path = FocusPath::Path(vec![0, 1]);
        // let eval = Eval::new(Context::default(), expr);
        // assert_eq!(eval.focus_path(), path);
    }
}
