use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::{Storage, Window};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn window() -> Result<Window> {
    web_sys::window().ok_or(anyhow!("No Window Found"))
}

pub fn local_storage() -> Result<Storage> {
    window()?
        .local_storage()
        .map_err(|err| anyhow!("Failed to get localStorage: {:?}", err))?
        .ok_or(anyhow!("No localStorage Found"))
}
