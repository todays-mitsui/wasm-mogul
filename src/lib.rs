#[macro_use]
mod browser;
mod calc;
mod context;
mod engine;
mod expr;
mod func;
mod parser;
mod style;
mod to_string;

use anyhow::Result;
use calc::{Eval, EvalStep};
use context::Context;
use engine::Engine;
use engine::Output;
use parser::{parse_command, parse_expr};
use serde::Serialize;
use std::fmt::Display;
pub use style::Style;
use style::{ECMAScriptStyle, Factor, LazyKStyle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct CalcResult {
    pub expr: String,
    pub steps: Box<[JsValue]>,
}

#[wasm_bindgen]
pub fn lambda_calculus(input: &str, style: Style) -> JsValue {
    log!("input: {}", input);
    let command = parse_command(input).expect("parse error");

    let context = Context::default();
    let mut engine = Engine::new(context);
    let output = engine.run(command);

    serde_wasm_bindgen::to_value(&JsOutput::from((&style, output))).unwrap()
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}

// ========================================================================== //

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum JsOutput {
    Del {
        input: String,       // Identifier
        result: Vec<String>, // Context
    },
    Update {
        input: String,       // Func
        result: Vec<String>, // Context
    },
    Eval {
        input: String, // Expr
        steps: Vec<JsEvalStep>,
    },
    Search {
        input: String,          // Identifier
        result: Option<String>, // Option<Func>
    },
    Global {
        result: Vec<String>, // Context
    },
    Unlambda {
        input: String,  // Expr
        result: String, // Expr
    },
}

impl From<(&Style, Output)> for JsOutput {
    fn from((style, output): (&Style, Output)) -> Self {
        let expr_to_string = |expr| {
            let _: &crate::expr::Expr = expr;
            match style {
                Style::ECMAScript => ECMAScriptStyle(expr).to_string(),
                Style::LazyK => LazyKStyle(expr).to_string(),
                _ => unreachable!(),
            }
        };
        let func_to_string = |func| {
            let _: &crate::func::Func = func;
            match style {
                Style::ECMAScript => ECMAScriptStyle(func).to_string(),
                Style::LazyK => LazyKStyle(func).to_string(),
                _ => unreachable!(),
            }
        };

        match output {
            Output::Del {
                input: id,
                result: context,
            } => Self::Del {
                input: id.to_string(),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func_to_string(func))
                    .collect(),
            },
            Output::Update {
                input: func,
                result: context,
            } => Self::Update {
                input: func_to_string(&func),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func_to_string(func))
                    .collect(),
            },
            Output::Eval { input: expr, steps } => Self::Eval {
                input: expr_to_string(&expr),
                steps: steps
                    .into_iter()
                    .map(|expr| JsEvalStep::from((style, expr)))
                    .collect(),
            },
            Output::Search { input: id, result } => Self::Search {
                input: id.to_string(),
                result: result.as_ref().map(|func| func_to_string(func)),
            },
            Output::Global { result: context } => Self::Global {
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func_to_string(func))
                    .collect(),
            },
            Output::Unlambda { input, result } => Self::Unlambda {
                input: expr_to_string(&input),
                result: expr_to_string(&result),
            },
        }
    }
}

#[derive(Serialize)]
pub struct JsEvalStep {
    expr: String, // Expr
}

impl From<(&Style, EvalStep)> for JsEvalStep {
    fn from((style, EvalStep { expr }): (&Style, EvalStep)) -> Self {
        Self {
            expr: match style {
                Style::ECMAScript => ECMAScriptStyle(&expr).to_string(),
                Style::LazyK => LazyKStyle(&expr).to_string(),
                _ => unreachable!(),
            },
        }
    }
}
