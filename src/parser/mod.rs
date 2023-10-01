mod ecmascript;
mod identifier;
mod lazy_k;
mod utils;

pub use ecmascript::parse_command as parse_command_with_ecmascript_style;
pub use ecmascript::parse_expr as parse_expr_with_ecmascript_style;
pub use lazy_k::parse_command as parse_command_with_lazy_k_style;
pub use lazy_k::parse_expr as parse_expr_with_lazy_k_style;
pub use lazy_k::{parse_command, parse_expr};
