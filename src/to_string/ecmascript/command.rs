use super::super::style::ECMAScriptStyle;
use crate::command::Command;
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
            Command::Info(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(e) => write!(f, "?? {}", ECMAScriptStyle(e)),
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
            Command::Info(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(e) => write!(f, "?? {}", ECMAScriptStyle(e)),
        }
    }
}

// ========================================================================== //

// TODO: テスト書く
