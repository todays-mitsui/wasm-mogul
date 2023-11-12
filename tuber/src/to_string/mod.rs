mod ecmascript;
mod lazy_k;

use crate::context::Context;
use crate::engine::Command;
use crate::expr::Expr;
use crate::func::Func;
use std::fmt::Display;

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lazy_k::command::to_string(self))
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lazy_k::context::to_string(self))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lazy_k::expression::to_string(self))
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lazy_k::function::to_string(self))
    }
}

// ========================================================================== //

#[derive(Copy, Clone, Debug)]
pub enum DisplayStyle {
    EcmaScript,
    LazyK,
}

pub trait Format {
    fn format(&self, style: &DisplayStyle) -> String;
}

impl Format for Command {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::command::to_string(self),
            DisplayStyle::LazyK => lazy_k::command::to_string(self),
        }
    }
}
impl Format for &Command {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::command::to_string(self),
            DisplayStyle::LazyK => lazy_k::command::to_string(self),
        }
    }
}

impl Format for Context {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::context::to_string(self),
            DisplayStyle::LazyK => lazy_k::context::to_string(self),
        }
    }
}
impl Format for &Context {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::context::to_string(self),
            DisplayStyle::LazyK => lazy_k::context::to_string(self),
        }
    }
}

impl Format for Expr {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::expression::to_string(self),
            DisplayStyle::LazyK => lazy_k::expression::to_string(self),
        }
    }
}
impl Format for &Expr {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::expression::to_string(self),
            DisplayStyle::LazyK => lazy_k::expression::to_string(self),
        }
    }
}

impl Format for Func {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::function::to_string(self),
            DisplayStyle::LazyK => lazy_k::function::to_string(self),
        }
    }
}
impl Format for &Func {
    fn format(&self, style: &DisplayStyle) -> String {
        match style {
            DisplayStyle::EcmaScript => ecmascript::function::to_string(self),
            DisplayStyle::LazyK => lazy_k::function::to_string(self),
        }
    }
}
