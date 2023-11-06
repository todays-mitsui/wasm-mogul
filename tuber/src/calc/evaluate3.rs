use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq)]
enum Focus {
    Done,
    Route(Vec<usize>),
}

impl Focus {
    fn push(&mut self, index: usize) {
        match self {
            Focus::Done => panic!("すでに簡約が完了しているため Focus を移動できない"),
            Focus::Route(route) => {
                route.push(index);
            }
        }
    }

    fn shift(&mut self) -> Option<usize> {
        match self {
            Focus::Done => panic!("すでに簡約が完了しているため index を取り出せない"),
            Focus::Route(route) if route.is_empty() => None,
            Focus::Route(route) => {
                route.rotate_left(1);
                route.pop()
            }
        }
    }

    fn into_focus(&mut self, context: &Context, expr: &mut Expr) {
        let mut focus = self.clone();

        match expr.walk_to(&mut focus) {
            None => panic!("道なき道を行く？"),
            Some((callee, args)) => {
                let maybe_arity = arity(context, callee).and_then(|arity| {
                    let len = args.len();
                    if len > 0 && len >= arity {
                        Some(arity)
                    } else {
                        None
                    }
                });
                match maybe_arity {
                    Some(_) => return,
                    None => {
                        for arg in args.into_iter().rev() {
                            let mut sub_focus = Focus::Route(Vec::new());
                            sub_focus.into_focus(context, arg);

                            if let Focus::Done = sub_focus {
                                continue;
                            } else {
                                self.join(sub_focus);
                                return;
                            }
                        }

                        *self = Focus::Done;
                    }
                }
            }
        }
    }

    fn join(&mut self, mut other: Focus) {
        match self {
            Focus::Done => return,
            Focus::Route(route) => match other {
                Focus::Done => {
                    *self = Focus::Done;
                }
                Focus::Route(other_route) => {
                    route.extend(other_route);
                }
            },
        }
    }
}

impl Expr {
    fn walk_to(&mut self, focus: &mut Focus) -> Option<(&mut Expr, Vec<&mut Expr>)> {
        match focus {
            Focus::Done => return None,
            Focus::Route(route) => {
                let mut args: Vec<Option<&mut Expr>> = Vec::new();
                let mut expr = self;
                while let Expr::Apply { lhs, rhs } = expr {
                    expr = lhs;
                    args.push(Some(rhs));
                }

                if route.is_empty() {
                    Some((
                        expr,
                        args.into_iter().map(|arg| arg.unwrap()).collect::<Vec<_>>(),
                    ))
                } else {
                    let index = focus.shift().unwrap();

                    match args.get_mut(index) {
                        None => panic!("道なき道を行く？"),
                        Some(maybe_expr) => maybe_expr.take().and_then(|expr| expr.walk_to(focus)),
                    }
                }
            }
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    #[test]
    fn test_focus_push() {
        let mut focus = Focus::Route(vec![0, 1, 2]);

        focus.push(3);
        assert_eq!(focus, Focus::Route(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_focus_shift() {
        let mut focus = Focus::Route(vec![0, 1, 2]);
        assert_eq!(focus.shift(), Some(0));
        assert_eq!(focus.shift(), Some(1));
        assert_eq!(focus.shift(), Some(2));
        assert_eq!(focus.shift(), None);
    }

    #[test]
    fn test_walk_to() {
        let mut expr = expr::a("f", expr::a("i", "x"));
        let mut focus = Focus::Route(vec![0]);

        let (callee, args) = expr.walk_to(&mut focus).unwrap();

        let mut expected_callee = expr::v("i");
        let mut expected_args = vec![expr::v("x")];

        assert_eq!(callee, &mut expected_callee);
        assert_eq!(args, expected_args.iter_mut().collect::<Vec<_>>());
    }
}
