use anyhow::{anyhow, Result};
use std::future::Future;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::{
    Document, HtmlButtonElement, HtmlDivElement, HtmlFormElement, HtmlInputElement, ScrollBehavior,
    ScrollToOptions, Window,
};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// ========================================================================== //

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

// ========================================================================== //

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(anyhow!("No Document Found"))
}

// ========================================================================== //

pub fn container() -> Result<HtmlDivElement> {
    document()?
        .get_element_by_id("container")
        .ok_or(anyhow!("No Container Found"))?
        .dyn_into::<HtmlDivElement>()
        .map_err(|elem| anyhow!("Failed to cast to HtmlDivElement: {:#?}", elem))
}

pub fn form() -> Result<HtmlFormElement> {
    document()?
        .get_element_by_id("form")
        .ok_or(anyhow!("No Form Found"))?
        .dyn_into::<HtmlFormElement>()
        .map_err(|elem| anyhow!("Failed to cast to HtmlFormElement: {:#?}", elem))
}

pub fn input() -> Result<HtmlInputElement> {
    document()?
        .get_element_by_id("src")
        .ok_or(anyhow!("No Input Found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|elem| anyhow!("Failed to cast to HtmlInputElement: {:#?}", elem))
}

pub fn submit() -> Result<HtmlButtonElement> {
    document()?
        .get_element_by_id("submit")
        .ok_or(anyhow!("No Submit Button Found"))?
        .dyn_into::<HtmlButtonElement>()
        .map_err(|elem| anyhow!("Failed to cast to HtmlButtonElement: {:#?}", elem))
}

// ========================================================================== //

pub fn scroll_to(top: f64) -> Result<()> {
    container()?.scroll_to_with_scroll_to_options(
        ScrollToOptions::new()
            .top(top)
            .behavior(ScrollBehavior::Smooth),
    );
    Ok(())
}
