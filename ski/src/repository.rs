use crate::browser::local_storage;
use anyhow::{anyhow, Result};
use tuber::{parse_update_or_delete, Command, Context, DisplayStyle, Func, Identifier};

const KEY_DISPLAY_STYLE: &str = "tuber_display_style";

pub fn get_display_style() -> Result<DisplayStyle> {
    let storage = local_storage()?;

    let display_style = storage
        .get_item(KEY_DISPLAY_STYLE)
        .map_err(|err| anyhow!("Failed to get display style from localStorage: {:?}", err))?;

    match display_style.as_ref().map(|s| s.as_str()) {
        None => Ok(DisplayStyle::EcmaScript),
        Some("ECMAScript") => Ok(DisplayStyle::EcmaScript),
        Some("Lazy_K") => Ok(DisplayStyle::LazyK),
        _ => Err(anyhow!("Invalid display style")),
    }
}

// ========================================================================== //

const KEY_FUNC_HISTORY: &str = "tuber_func_history";

pub fn get_context() -> Result<Context> {
    let mut context = Context::default();
    for command in get_func_history()?.into_iter() {
        match command {
            Command::Update(func) => context.def(func),
            Command::Del(id) => {
                context.del(&id);
            }
            _ => unreachable!(),
        }
    }
    Ok(context)
}

fn get_func_history() -> Result<Vec<Command>> {
    let storage = local_storage()?;

    let history_string = storage
        .get_item(KEY_FUNC_HISTORY)
        .map_err(|err| anyhow!("Failed to get func history from localStorage: {:?}", err))?;

    if history_string.is_none() {
        return Ok(Vec::new());
    }

    let mut commands = Vec::new();
    for command_str in history_string.unwrap().split('\n') {
        if command_str.trim().is_empty() {
            continue;
        }

        let command = parse_update_or_delete(command_str)?;
        commands.push(command);
    }

    Ok(commands)
}

pub fn push_history_def(func: &Func) -> Result<()> {
    let storage = local_storage()?;

    let history_string = storage
        .get_item(KEY_FUNC_HISTORY)
        .map_err(|err| anyhow!("Failed to get func history from localStorage: {:?}", err))?;

    let mut history_string = history_string.unwrap_or_default();
    history_string = history_string + "\n" + func.to_string().as_str();

    storage
        .set_item(KEY_FUNC_HISTORY, history_string.as_str())
        .map_err(|err| anyhow!("Failed to set func history to localStorage: {:?}", err))
}

pub fn push_history_del(id: &Identifier) -> Result<()> {
    let storage = local_storage()?;

    let history_string = storage
        .get_item(KEY_FUNC_HISTORY)
        .map_err(|err| anyhow!("Failed to get func history from localStorage: {:?}", err))?;

    let mut history_string = history_string.unwrap_or_default();
    history_string = history_string + "\n" + format!("{0} = {0}", id).as_str();

    storage
        .set_item(KEY_FUNC_HISTORY, history_string.as_str())
        .map_err(|err| anyhow!("Failed to set func history to localStorage: {:?}", err))
}

pub fn clear_history() -> Result<()> {
    let storage = local_storage()?;

    storage
        .set_item(KEY_FUNC_HISTORY, "")
        .map_err(|err| anyhow!("Failed to set func history to localStorage: {:?}", err))
}

// ========================================================================== //

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

//     wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     fn test_get_display_style() {
//         // TODO: テスト書く
//     }
// }
