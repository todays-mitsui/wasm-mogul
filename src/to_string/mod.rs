mod ecmascript;
mod lazy_k;

use crate::context::Context;
use crate::engine::Command;
use crate::expr::Expr;
use crate::func::Func;
use crate::style::LazyKStyle;
use std::fmt::Display;

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LazyKStyle(self))
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LazyKStyle(self))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LazyKStyle(self))
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LazyKStyle(self))
    }
}
