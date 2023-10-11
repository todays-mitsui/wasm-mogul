use crate::browser::local_storage;
use crate::context::Context;
use crate::func::Func;
use crate::parser::parse_func;
use crate::style::{DisplayStyle, ECMAScriptStyle};
use anyhow::{anyhow, Result};

const KEY_DISPLAY_STYLE: &str = "tuber_display_style";

pub fn get_display_style() -> Result<DisplayStyle> {
    let storage = local_storage()?;

    let display_style = storage
        .get_item(KEY_DISPLAY_STYLE)
        .map_err(|err| anyhow!("Failed to get display style from localStorage: {:?}", err))?;

    match display_style.as_ref().map(|s| s.as_str()) {
        None => Ok(DisplayStyle::ECMAScript),
        Some("ECMAScript") => Ok(DisplayStyle::ECMAScript),
        Some("Lazy_K") => Ok(DisplayStyle::LazyK),
        _ => Err(anyhow!("Invalid display style")),
    }
}

// ========================================================================== //

const KEY_FUNC_HISTORY: &str = "tuber_func_history";

pub fn get_context() -> Result<Context> {
    let mut context = Context::default();
    for func in get_func_history()?.into_iter() {
        context.def(func);
    }
    Ok(context)
}

fn get_func_history() -> Result<Vec<Func>> {
    let storage = local_storage()?;

    let history_string = storage
        .get_item(KEY_FUNC_HISTORY)
        .map_err(|err| anyhow!("Failed to get func history from localStorage: {:?}", err))?;

    if history_string.is_none() {
        return Ok(Vec::new());
    }

    let mut funcs = Vec::new();
    for func_str in history_string.unwrap().split('\n') {
        if func_str.trim().is_empty() {
            continue;
        }

        let func = parse_func(func_str)?;
        funcs.push(func);
    }

    Ok(funcs)
}

pub fn push_func_history(func: &Func) -> Result<()> {
    let storage = local_storage()?;

    let history_string = storage
        .get_item(KEY_FUNC_HISTORY)
        .map_err(|err| anyhow!("Failed to get func history from localStorage: {:?}", err))?;

    let mut history_string = history_string.unwrap_or_default();
    history_string = history_string + "\n" + ECMAScriptStyle(func).to_string().as_str();

    storage
        .set_item(KEY_FUNC_HISTORY, history_string.as_str())
        .map_err(|err| anyhow!("Failed to set func history to localStorage: {:?}", err))
}
