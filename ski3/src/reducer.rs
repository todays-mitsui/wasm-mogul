use crate::context::Context;
use crate::expression::Expr;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, ecmascript_format, lazy_k_format, Format, Tag};
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
        let tuber_reduce_result = self.0.next();
        JsNext {
            done: tuber_reduce_result.is_none(),
            value: tuber_reduce_result.map(|result| {
                ReduceResult::new(
                    result.step,
                    result.expr.into(),
                    result.reduced_path,
                    self.0.reducible_path(),
                    tuber::DisplayStyle::EcmaScript,
                )
            }),
        }
    }

    #[wasm_bindgen(js_name = hasNext)]
    pub fn has_next(&self) -> bool {
        self.reducible_path().is_some()
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
struct JsNext {
    done: bool,
    value: Option<ReduceResult>,
}

// ========================================================================== //

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
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
    ) -> Result<Self, Error> {
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
struct Formed {
    expr: String,
    reduced_range: Range,
    reducible_range: Option<Range>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct Range(std::ops::Range<usize>);

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ReducibleRange {
    entire: Range,
    callee: Range,
    args: Vec<Range>,
}

fn format_expr(
    expr: &tuber::Expr,
    reduced_path: &tuber::Path,
    reducible_path: &Option<tuber::Path>,
    display_style: tuber::DisplayStyle,
) -> Result<Formed, Error> {
    let mut paths = vec![reduced_path];
    if let Some(reducible_path) = reducible_path {
        paths.push(reducible_path);
    }

    let formed = match display_style {
        DisplayStyle::EcmaScript => ecmascript_format(expr, &paths),
        DisplayStyle::LazyK => lazy_k_format(expr),
    };

    let reduced_range =
        reduced_path_to_range(&formed.mapping, &reduced_path).ok_or(Error::InvalidRange)?;
    let reducible_range =
        reducible_path.and_then(|path| reduciblePath_path_to_range(&formed.mapping, &path));

    Ok(Formed {
        expr: formed.expr,
        reduced_range,
        reducible_range,
    })
}

fn reduced_path_to_range(mapping: &[Tag], path: &tuber::Path) -> Result<Range, Error> {
    path.range(mapping).ok_or(Error::InvalidRange)?.into()
}

fn reduciblePath_path_to_range(
    mapping: &[Tag],
    path: &tuber::Path,
) -> Result<ReducibleRange, Error> {
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
        .ok_or(Error::InvalidRange) // Tuber 側で Error 返すようにしたほうがいい
        .map(|range| range.into())?;

    let callee = callee_path
        .range(mapping)
        .ok_or(Error::InvalidRange)
        .map(|range| range.into())?;

    let mut args = Vec::new();
    for arg_path in args_path {
        let arg_range = arg_path
            .range(mapping)
            .ok_or(Error::InvalidRange)
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
