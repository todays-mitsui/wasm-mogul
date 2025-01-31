use crate::expression::Expr;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Aliases(HashMap<Identifier, Expr>);

impl From<tuber::Aliases> for Aliases {
    fn from(tuber_aliases: tuber::Aliases) -> Aliases {
        Aliases(
            tuber_aliases
                .into_iter()
                .map(|(id, func)| (id.as_str().into(), func.into()))
                .collect::<HashMap<Identifier, Expr>>(),
        )
    }
}

impl From<Aliases> for tuber::Aliases {
    fn from(ski_aliases: Aliases) -> tuber::Aliases {
        ski_aliases
            .0
            .into_iter()
            .map(|(id, expr)| (id.into(), expr.into()))
            .collect::<HashMap<tuber::Identifier, tuber::Expr>>()
            .into()
    }
}
