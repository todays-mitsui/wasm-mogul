use crate::engine::Command;
use crate::style::ECMAScriptStyle;
use std::fmt::Display;

impl Display for ECMAScriptStyle<'_, Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", ECMAScriptStyle(func)),
            Command::Eval(e) => write!(f, "{}", ECMAScriptStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", ECMAScriptStyle(e)),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, ECMAScriptStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, ECMAScriptStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), ECMAScriptStyle(e))
            }
        }
    }
}

impl Display for ECMAScriptStyle<'_, &Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", ECMAScriptStyle(func)),
            Command::Eval(e) => write!(f, "{}", ECMAScriptStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", ECMAScriptStyle(e)),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, ECMAScriptStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, ECMAScriptStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), ECMAScriptStyle(e))
            }
        }
    }
}

// ========================================================================== //

// TODO: テスト書く
