use crate::function::Func;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify_next::Tsify;
use tuber;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Context(HashMap<Identifier, Func>);

impl From<tuber::Context> for Context {
    fn from(tuber_context: tuber::Context) -> Context {
        Context(
            tuber_context
                .into_iter()
                .map(|(id, func)| (id.as_str().into(), func.into()))
                .collect::<HashMap<Identifier, Func>>(),
        )
    }
}

impl From<Context> for tuber::Context {
    fn from(ski_context: Context) -> tuber::Context {
        ski_context
            .into_iter()
            .map(|(id, func)| (id.into(), func.into()))
            .collect::<HashMap<tuber::Identifier, tuber::Func>>()
            .into()
    }
}
