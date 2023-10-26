#[macro_use]
mod browser;
mod repository;
mod style;

use anyhow::Result;
use repository::{get_context, get_display_style, push_history_def, push_history_del};
use serde::Serialize;
use style::{DisplayStyle, ECMAScriptStyle, LazyKStyle};
use tuber::calc::EvalStep;
use tuber::engine::Engine;
use tuber::engine::Output;
use tuber::parser::parse_command as parser_parse_command;
use tuber::parser::parse_expr as parser_parse_expr;
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
    let func_to_string = |func| {
        let _: &crate::func::Func = func;
        match style {
            DisplayStyle::ECMAScript => ECMAScriptStyle(func).to_string(),
            DisplayStyle::LazyK => LazyKStyle(func).to_string(),
            _ => unreachable!(),
        }
    };

    let context = get_context().expect("get context error");
    let vec: Vec<JsValue> = context
        .to_vec()
        .iter()
        .map(|func| JsValue::from_str(&func_to_string(func)))
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
        let expr_to_string = |expr| {
            let _: &crate::expr::Expr = expr;
            match style {
                DisplayStyle::ECMAScript => ECMAScriptStyle(expr).to_string(),
                DisplayStyle::LazyK => LazyKStyle(expr).to_string(),
                _ => unreachable!(),
            }
        };
        let func_to_string = |func| {
            let _: &crate::func::Func = func;
            match style {
                DisplayStyle::ECMAScript => ECMAScriptStyle(func).to_string(),
                DisplayStyle::LazyK => LazyKStyle(func).to_string(),
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
            Output::Context { result: context } => Self::Context {
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

impl From<(&DisplayStyle, EvalStep)> for JsEvalStep {
    fn from((style, EvalStep { expr }): (&DisplayStyle, EvalStep)) -> Self {
        Self {
            expr: match style {
                DisplayStyle::ECMAScript => ECMAScriptStyle(&expr).to_string(),
                DisplayStyle::LazyK => LazyKStyle(&expr).to_string(),
                _ => unreachable!(),
            },
        }
    }
}
