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
}

impl Expr {
    fn walk_to(&mut self, focus: &mut Focus) -> Option<&mut Expr> {
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
                    Some(expr)
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

struct Args<'a>(Vec<Option<&'a mut Expr>>);

impl<'a> Args<'a> {
    fn new() -> Self {
        Args(Vec::new())
    }

    fn push(&mut self, expr: &'a mut Expr) {
        self.0.push(Some(expr));
    }

    fn nth(&mut self, index: usize) -> Option<&mut Expr> {
        self.0.get_mut(index)?.take()
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

        let sub_expr = expr.walk_to(&mut focus);
        let expected = expr::v("i");

        assert!(sub_expr.is_some());
        assert_eq!(*sub_expr.unwrap(), expected);
    }
}
