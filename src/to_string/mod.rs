mod ecmascript;
mod lazy_k;
mod style;

use crate::command::Command;
use crate::context::Context;
use crate::expr::Expr;
use crate::func::Func;
use std::fmt::Display;
pub use style::{ECMAScriptStyle, LazyKStyle};

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
