use crate::function::Func;
use crate::identifier::Identifier;
use serde_wasm_bindgen;
use std::collections::HashMap;
use tsify_next::declare;
use tuber;
use wasm_bindgen::prelude::*;

#[declare]
pub type Context = HashMap<Identifier, Func>;

pub fn to_tuber_context(ski_context: Context) -> tuber::Context {
    ski_context
        .into_iter()
        .map(|(id, func)| (id.into(), func.into()))
        .collect::<HashMap<tuber::Identifier, tuber::Func>>()
        .into()
}

pub fn from_tuber_context(tuber_context: tuber::Context) -> Context {
    tuber_context
        .into_iter()
        .map(|(id, func)| (id.as_str().into(), func.into()))
        .collect::<HashMap<Identifier, Func>>()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_PRINT_CONTEXT: &'static str = "export function printContext(context: Context): string;";

#[wasm_bindgen(js_name = printContext)]
pub fn print_context(context: JsValue) -> String {
    let context = serde_wasm_bindgen::from_value::<Context>(context);
    format!("{:?}", context)
}
