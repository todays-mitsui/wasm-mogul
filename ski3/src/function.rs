use crate::display_style::DisplayStyle;
use crate::expression::Expr;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, Format};
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Func(
    /* name   */ Identifier,
    /* params */ Vec<Identifier>,
    /* body   */ Expr,
);

impl From<tuber::Func> for Func {
    fn from(tuber_func: tuber::Func) -> Func {
        Func(
            tuber_func.name().to_string(),
            tuber_func
                .params()
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
            Expr::from(tuber_func.body().clone()),
        )
    }
}

impl From<Func> for tuber::Func {
    fn from(ski_func: Func) -> tuber::Func {
        tuber::Func::new(
            ski_func.0.into(),
            ski_func.1.into_iter().map(|id| id.into()).collect(),
            ski_func.2.into(),
        )
    }
}

#[wasm_bindgen(js_name = renderFunc)]
#[allow(non_snake_case)]
pub fn render_func(func: Func, displayStyle: DisplayStyle) -> String {
    let tuber_func: tuber::Func = func.into();
    let tuber_display_style: tuber::DisplayStyle = displayStyle.into();
    tuber_func.format(&tuber_display_style)
}
