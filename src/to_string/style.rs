use crate::command::Command;
use crate::context::Context;
use crate::expr::Expr;
use crate::func::Func;

pub trait Factor {}

impl Factor for Command {}
impl Factor for &Command {}
impl Factor for Context {}
impl Factor for &Context {}
impl Factor for Expr {}
impl Factor for &Expr {}
impl Factor for Func {}
impl Factor for &Func {}

#[derive(PartialEq, Debug)]
pub struct LazyKStyle<'a, F: Factor>(pub &'a F);

#[derive(PartialEq, Debug)]
pub struct ECMAScriptStyle<'a, F: Factor>(pub &'a F);
