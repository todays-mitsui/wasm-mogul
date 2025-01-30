use crate::context::Context;
use crate::display_style::DisplayStyle;
use crate::expression::Expr;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, ecmascript_format, lazy_k_format, Tag};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Reducer {
    reducer: tuber::Reducer,
    reducible_path: Option<tuber::Path>,
    display_style: tuber::DisplayStyle,
}

#[wasm_bindgen]
impl Reducer {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(context: Context, expr: Expr, displayStyle: Option<DisplayStyle>) -> Self {
        let tuber_context = context.into();
        let tuber_expr = expr.into();
        let reducer = tuber::Reducer::new(tuber_context, tuber_expr, false);
        let display_style = displayStyle
            .map(tuber::DisplayStyle::from)
            .unwrap_or(tuber::DisplayStyle::EcmaScript);
        Self {
            reducible_path: reducer.reducible_path(),
            reducer,
            display_style,
        }
    }

    #[wasm_bindgen(getter = displayStyle)]
    pub fn get_display_style(&self) -> DisplayStyle {
        (&self.display_style).into()
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(setter = displayStyle)]
    pub fn set_display_style(&mut self, displayStyle: DisplayStyle) {
        self.display_style = displayStyle.into();
    }

    #[wasm_bindgen(getter)]
    pub fn formed(&self) -> Result<FormedExpr, JsError> {
        let expr = self.reducer.expr();
        format_expr(&expr, &self.reducible_path, &self.display_style)
    }

    #[wasm_bindgen(getter = hasNext)]
    pub fn has_next(&self) -> bool {
        self.reducible_path.is_some()
    }

    #[wasm_bindgen(js_name = next)]
    pub fn js_next(&mut self) -> Result<IteratorResult, JsError> {
        let tuber_reduce_result = self.reducer.next();
        self.reducible_path = self.reducer.reducible_path();

        let ski_reduce_result = match tuber_reduce_result {
            Some(result) => Some(ReduceResult::new(
                result.step,
                result.expr.clone(),
                result.reduced_path,
                &self.reducible_path,
                &self.display_style,
            )?),
            None => None,
        };

        Ok(IteratorResult {
            done: ski_reduce_result.is_none(),
            value: ski_reduce_result,
        })
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct IteratorResult {
    done: bool,
    value: Option<ReduceResult>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ExprRange(std::ops::Range<usize>);

// ========================================================================== //

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct FormedExpr {
    expr: String,
    reducible_range: Option<ReducibleRange>,
}

fn format_expr(
    expr: &tuber::Expr,
    reducible_path: &Option<tuber::Path>,
    display_style: &tuber::DisplayStyle,
) -> Result<FormedExpr, JsError> {
    let mut paths: Vec<&tuber::Path> = Vec::new();
    if let Some(reducible_path) = reducible_path {
        paths.push(reducible_path);
    }

    let formed = match display_style {
        tuber::DisplayStyle::EcmaScript => ecmascript_format(expr, &paths),
        tuber::DisplayStyle::LazyK => lazy_k_format(expr),
    };

    let reducible_range = match reducible_path {
        None => None,
        Some(reducible_path) => Some(reducible_path_path_to_range(
            &formed.mapping,
            reducible_path,
        )?),
    };

    Ok(FormedExpr {
        expr: formed.expr,
        reducible_range,
    })
}

// ========================================================================== //

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
struct ReduceResult {
    step: usize,
    expr: Expr,
    formed: FormedReducedExpr,
}

impl ReduceResult {
    fn new(
        step: usize,
        expr: tuber::Expr,
        reduced_path: tuber::Path,
        reducible_path: &Option<tuber::Path>,
        display_style: &tuber::DisplayStyle,
    ) -> Result<Self, JsError> {
        let formed = format_reduced_expr(&expr, &reduced_path, reducible_path, display_style)?;
        Ok(Self {
            step,
            expr: expr.into(),
            formed,
        })
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
struct FormedReducedExpr {
    expr: String,
    reduced_range: ExprRange,
    reducible_range: Option<ReducibleRange>,
}

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

fn format_reduced_expr(
    expr: &tuber::Expr,
    reduced_path: &tuber::Path,
    reducible_path: &Option<tuber::Path>,
    display_style: &tuber::DisplayStyle,
) -> Result<FormedReducedExpr, JsError> {
    let mut paths = vec![reduced_path];
    if let Some(reducible_path) = reducible_path {
        paths.push(reducible_path);
    }

    let formed = match display_style {
        tuber::DisplayStyle::EcmaScript => ecmascript_format(expr, &paths),
        tuber::DisplayStyle::LazyK => lazy_k_format(expr),
    };

    let reduced_range = reduced_path_to_range(&formed.mapping, reduced_path)?;
    let reducible_range = match reducible_path {
        None => None,
        Some(reducible_path) => Some(reducible_path_path_to_range(
            &formed.mapping,
            reducible_path,
        )?),
    };

    Ok(FormedReducedExpr {
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
