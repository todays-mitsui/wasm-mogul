use crate::context::Context;
use crate::engine::Command;
use crate::expr::Expr;
use crate::func::Func;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum DisplayStyle {
    ECMAScript = "ECMAScript",
    LazyK = "Lazy_K",
}
