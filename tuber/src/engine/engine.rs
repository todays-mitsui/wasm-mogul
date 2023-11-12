use super::Command;
use crate::calc::{
    expand, unlambda_iota, unlambda_recursive, unlambda_recursive_, unlambda_shallow, Eval,
    RecursiveStrategy,
};
use crate::context::Context;
use crate::expr::{Expr, Identifier};
use crate::func::Func;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn run(self, command: Command) -> RunResult {
        match command {
            Command::Del(id) => {
                let mut context = self.context;
                context.del(&id);
                RunResult::Del {
                    input: id,
                    result: context,
                }
            }

            Command::Update(func) => {
                let mut context = self.context;
                context.def(func.clone());
                RunResult::Update {
                    input: func,
                    result: context,
                }
            }

            Command::Eval(expr) => {
                let eval = Eval::new(self.context, expr.clone());

                RunResult::Eval { input: expr, eval }
            }

            // Command::EvalLast(e) => {
            //     match &self.display_style {
            //         Style::LazyK => println!("{}", LazyKStyle(&e)),
            //         Style::EcmaScript => println!("{}", EcmaScriptStyle(&e)),
            //         _ => unreachable!(),
            //     }

            //     let mut steps = Eval::new(e, &self.context);
            //     if let (Some(e), _continue) = steps.eval_last(100) {
            //         println!("→ ...");
            //         match &self.display_style {
            //             Style::LazyK => println!("→ {}", LazyKStyle(&e)),
            //             Style::EcmaScript => println!("→ {}", EcmaScriptStyle(&e)),
            //             _ => unreachable!(),
            //         }
            //     } else {
            //         // TODO
            //     }
            // }
            Command::Search(id) => self.context.get(&id).map_or(
                RunResult::Search {
                    input: id.clone(),
                    result: None,
                },
                |func| RunResult::Search {
                    input: id.clone(),
                    result: Some(func.clone()),
                },
            ),

            Command::Context => RunResult::Context {
                result: self.context,
            },

            Command::Unlambda(level, e) => RunResult::Unlambda {
                input: e.clone(),
                level,
                result: match level {
                    1 => expand(&self.context, e),
                    2 => unlambda_recursive(&self.context, e),
                    3 => unlambda_recursive_(&RecursiveStrategy::SK, &self.context, e),
                    4 => unlambda_iota(&self.context, e),
                    _ => panic!("not implemented"),
                },
            },

            _ => panic!("not implemented"),
        }
    }
}

pub enum RunResult {
    Del {
        input: Identifier,
        result: Context,
    },
    Update {
        input: Func,
        result: Context,
    },
    Eval {
        input: Expr,
        eval: Eval,
    },
    Search {
        input: Identifier,
        result: Option<Func>,
    },
    Context {
        result: Context,
    },
    Unlambda {
        input: Expr,
        level: u8,
        result: Expr,
    },
}
