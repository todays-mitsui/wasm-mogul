use crate::context::Context;
use crate::expression::Expr;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, ecmascript_format, lazy_k_format, Tag};
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
    pub fn js_next(&mut self) -> Result<IteratorResult, JsError> {
        let tuber_reduce_result = self.0.next();

        let ski_reduce_result = match tuber_reduce_result {
            Some(result) => Some(ReduceResult::new(
                result.step,
                result.expr.clone(),
                result.reduced_path.clone(),
                self.0.reducible_path(),
                tuber::DisplayStyle::EcmaScript,
            )?),
            None => None,
        };

        Ok(IteratorResult {
            done: ski_reduce_result.is_none(),
            value: ski_reduce_result,
        })
    }

    #[wasm_bindgen(js_name = hasNext)]
    pub fn has_next(&self) -> bool {
        self.reducible_path().is_some()
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct IteratorResult {
    done: bool,
    value: Option<ReduceResult>,
}

// ========================================================================== //

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
struct ReduceResult {
    step: usize,
    expr: Expr,
    reduced_path: Path,
    reducible_path: Option<Path>,
    formed: Formed,
}

impl ReduceResult {
    fn new(
        step: usize,
        expr: tuber::Expr,
        reduced_path: tuber::Path,
        reducible_path: Option<tuber::Path>,
        display_style: tuber::DisplayStyle,
    ) -> Result<Self, JsError> {
        let formed = format_expr(&expr, &reduced_path, &reducible_path, display_style)?;
        Ok(Self {
            step,
            expr: expr.into(),
            reduced_path: Path::from(&reduced_path),
            reducible_path: reducible_path.map(|path| Path::from(&path)),
            formed,
        })
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
struct Formed {
    expr: String,
    reduced_range: ExprRange,
    reducible_range: Option<ReducibleRange>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ExprRange(std::ops::Range<usize>);

impl From<std::ops::Range<usize>> for ExprRange {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self(range)
    }
}

impl From<ExprRange> for std::ops::Range<usize> {
    fn from(range: ExprRange) -> Self {
        range.0
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ReducibleRange {
    entire: ExprRange,
    callee: ExprRange,
    args: Vec<ExprRange>,
}

fn format_expr(
    expr: &tuber::Expr,
    reduced_path: &tuber::Path,
    reducible_path: &Option<tuber::Path>,
    display_style: tuber::DisplayStyle,
) -> Result<Formed, JsError> {
    let mut paths = vec![reduced_path];
    if let Some(reducible_path) = reducible_path {
        paths.push(reducible_path);
    }

    let formed = match display_style {
        tuber::DisplayStyle::EcmaScript => ecmascript_format(expr, &paths),
        tuber::DisplayStyle::LazyK => lazy_k_format(expr),
    };

    let reduced_range = reduced_path_to_range(&formed.mapping, &reduced_path)?;
    let reducible_range = match reducible_path {
        None => None,
        Some(reducible_path) => Some(reducible_path_path_to_range(
            &formed.mapping,
            reducible_path,
        )?),
    };

    Ok(Formed {
        expr: formed.expr,
        reduced_range,
        reducible_range,
    })
}

fn reduced_path_to_range(mapping: &[Tag], path: &tuber::Path) -> Result<ExprRange, JsError> {
    match path.range(mapping) {
        Some(range) => Ok(range.into()),
        None => Err(JsError::new("InvalidRange")),
    }
}

fn reducible_path_path_to_range(
    mapping: &[Tag],
    path: &tuber::Path,
) -> Result<ReducibleRange, JsError> {
    let arity: usize = path.get_arity();

    let mut callee_path = path.clone();
    callee_path.set_arity(0);

    let args_path = (0..arity).map(|index| {
        let mut path = path.clone();
        path.set_arity(index + 1);
        path.last_arg();
        path
    });

    let entire = path
        .range(mapping)
        .ok_or(JsError::new("InvalidRange")) // Tuber 側で JsError 返すようにしたほうがいい
        .map(|range| range.into())?;

    let callee = callee_path
        .range(mapping)
        .ok_or(JsError::new("InvalidRange"))
        .map(|range| range.into())?;

    let mut args = Vec::new();
    for arg_path in args_path {
        let arg_range = arg_path
            .range(mapping)
            .ok_or(JsError::new("InvalidRange"))
            .map(|range| range.into())?;
        args.push(arg_range);
    }

    Ok(ReducibleRange {
        entire,
        callee,
        args,
    })
}

// ========================================================================== //

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Path(Vec<usize>);

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

impl From<tuber::Path> for Path {
    fn from(tuber_path: tuber::Path) -> Self {
        tuber_path.into()
    }
}

impl From<Path> for tuber::Path {
    fn from(ski_path: Path) -> Self {
        ski_path.into()
    }
}
