use crate::context::Context;
use crate::expression::Expr;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, Format};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Reducer(tuber::Reducer);

#[wasm_bindgen]
impl Reducer {
    #[wasm_bindgen(constructor)]
    pub fn new(context: Context, expr: Expr) -> Self {
        let tuber_context = context.into();
        let tuber_expr = expr.into();
        Self(tuber::Reducer::new(tuber_context, tuber_expr))
    }

    #[wasm_bindgen(js_name = reduciblePath)]
    pub fn reducible_path(&self) -> Option<Path> {
        self.0
            .reducible_path()
            .map(|tuber_path| Path::from(&tuber_path))
    }

    #[wasm_bindgen(js_name = next)]
    pub fn js_next(&mut self) -> JsNext {
        let next = self.0.next();
        JsNext {
            done: next.is_none(),
            value: next.map(|result| result.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct JsNext {
    done: bool,
    value: Option<ReduceResult>,
}

// ========================================================================== //

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ReduceResult {
    step: usize,
    expr: Expr,
    reduced_path: String,
}

impl From<tuber::ReduceResult> for ReduceResult {
    fn from(result: tuber::ReduceResult) -> Self {
        Self {
            step: result.step,
            expr: result.expr.into(),
            reduced_path: "".to_string(), //result.reduced_path.to_string(),
        }
    }
}

// impl From<ReduceResult> for tuber::ReduceResult {
//     fn from(result: ReduceResult) -> Self {
//         Self {
//             step: result.step,
//             expr: result.expr.into(),
//             reduced_path: result.reduced_path.into(),
//         }
//     }
// }

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct Path(Vec<usize>);

impl From<&tuber::Path> for Path {
    fn from(tuber_path: &tuber::Path) -> Self {
        let mut indices = Vec::new();
        let mut path = tuber_path;
        loop {
            match path {
                tuber::Path::Arg(index, next) => {
                    indices.push(*index);
                    path = next;
                }
                tuber::Path::Callee(arity) => {
                    indices.push(*arity);
                    break;
                }
            }
        }
        Path(indices)
    }
}

impl From<&Path> for tuber::Path {
    fn from(ski_path: &Path) -> Self {
        let mut indices = ski_path.0.iter().rev();
        let mut path = tuber::Path::Callee(*indices.next().unwrap());
        for index in indices {
            path = tuber::Path::Arg(*index, Box::new(path));
        }
        path
    }
}
