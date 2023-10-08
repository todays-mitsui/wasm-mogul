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
pub use style::Style;
use style::{ECMAScriptStyle, LazyKStyle};
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

    serde_wasm_bindgen::to_value(&JsOutput::from(output)).unwrap()
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

impl From<Output> for JsOutput {
    fn from(output: Output) -> Self {
        match output {
            Output::Del {
                input: id,
                result: context,
            } => Self::Del {
                input: id.to_string(),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.to_string())
                    .collect(),
            },
            Output::Update {
                input: func,
                result: context,
            } => Self::Update {
                input: func.to_string(),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.to_string())
                    .collect(),
            },
            Output::Eval { input: expr, steps } => Self::Eval {
                input: expr.to_string(),
                steps: steps.into_iter().map(JsEvalStep::from).collect(),
            },
            Output::Search { input: id, result } => Self::Search {
                input: id.to_string(),
                result: result.map(|func| func.to_string()),
            },
            Output::Global { result: context } => Self::Global {
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.to_string())
                    .collect(),
            },
            Output::Unlambda { input, result } => Self::Unlambda {
                input: input.to_string(),
                result: result.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct JsEvalStep {
    expr: String, // Expr
}

impl From<EvalStep> for JsEvalStep {
    fn from(EvalStep { expr }: EvalStep) -> Self {
        Self {
            expr: LazyKStyle(&expr).to_string(),
        }
    }
}
