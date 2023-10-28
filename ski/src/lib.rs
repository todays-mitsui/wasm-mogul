#[macro_use]
mod browser;
mod repository;
mod style;

use anyhow::Result;
use repository::{get_context, get_display_style, push_history_def, push_history_del};
use serde::Serialize;
use tuber::parse_command as parser_parse_command;
use tuber::parse_expr as parser_parse_expr;
use tuber::{DisplayStyle, Engine, EvalStep, Format, Output};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct CalcResult {
    pub expr: String,
    pub steps: Box<[JsValue]>,
}

#[wasm_bindgen]
pub fn execute(input: &str) -> JsValue {
    let context = get_context().expect("get context error");
    let command = parser_parse_command(input).expect("parse error");

    let mut engine = Engine::new(context);
    let output = engine.run(command);

    match &output {
        Output::Update {
            input: func,
            result: _,
        } => push_history_def(&func),
        Output::Del {
            input: id,
            result: _,
        } => push_history_del(id),
        _ => Ok(()), // 何もしない
    }
    .expect("push func history error");

    let style = get_display_style().expect("get display style error");
    serde_wasm_bindgen::to_value(&JsOutput::from((&style, output))).unwrap()
}

#[wasm_bindgen]
pub fn context() -> Box<[JsValue]> {
    let style = get_display_style().expect("get display style error");

    let context = get_context().expect("get context error");
    let vec: Vec<JsValue> = context
        .to_vec()
        .iter()
        .map(|func| JsValue::from_str(func.format(&style).as_str()))
        .collect();

    vec.into_boxed_slice()
}

#[wasm_bindgen]
pub fn parse_command(input: &str) {
    let command = parser_parse_command(input).expect("parse error");
    log!("command: {:?}", command);
}

#[wasm_bindgen]
pub fn parse_expr(input: &str) {
    let expr = parser_parse_expr(input).expect("parse error");
    log!("expr: {:?}", expr);
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
    Context {
        result: Vec<String>, // Context
    },
    Unlambda {
        input: String,  // Expr
        result: String, // Expr
    },
}

impl From<(&DisplayStyle, Output)> for JsOutput {
    fn from((style, output): (&DisplayStyle, Output)) -> Self {
        match output {
            Output::Del {
                input: id,
                result: context,
            } => Self::Del {
                input: id.to_string(),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.format(style))
                    .collect(),
            },
            Output::Update {
                input: func,
                result: context,
            } => Self::Update {
                input: func.format(style),
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.format(style))
                    .collect(),
            },
            Output::Eval { input: expr, steps } => Self::Eval {
                input: expr.format(style),
                steps: steps
                    .into_iter()
                    .map(|expr| JsEvalStep::from((style, expr)))
                    .collect(),
            },
            Output::Search { input: id, result } => Self::Search {
                input: id.to_string(),
                result: result.as_ref().map(|func| func.format(style)),
            },
            Output::Context { result: context } => Self::Context {
                result: context
                    .to_vec()
                    .iter()
                    .map(|func| func.format(style))
                    .collect(),
            },
            Output::Unlambda { input, result } => Self::Unlambda {
                input: input.format(style),
                result: result.format(style),
            },
        }
    }
}

#[derive(Serialize)]
pub struct JsEvalStep {
    expr: String, // Expr
}

impl From<(&DisplayStyle, EvalStep)> for JsEvalStep {
    fn from((style, EvalStep { expr }): (&DisplayStyle, EvalStep)) -> Self {
        Self {
            expr: expr.format(style),
        }
    }
}
