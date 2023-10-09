use super::Command;
use super::Output;
use crate::calc::{unlambda_shallow, Eval};
use crate::context::Context;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn run(&mut self, command: Command) -> Output {
        match command {
            Command::Del(id) => {
                self.context.del(&id);
                Output::Del {
                    input: id,
                    result: self.context.clone(),
                }
            }

            Command::Update(func) => {
                self.context.def(func.clone());
                Output::Update {
                    input: func,
                    result: self.context.clone(),
                }
            }

            Command::Eval(e) => {
                let eval = Eval::new(e.clone(), &self.context);
                let steps = eval.take(1000).collect::<Vec<_>>();

                Output::Eval { input: e, steps }
            }

            // Command::EvalLast(e) => {
            //     match &self.display_style {
            //         Style::LazyK => println!("{}", LazyKStyle(&e)),
            //         Style::ECMAScript => println!("{}", ECMAScriptStyle(&e)),
            //         _ => unreachable!(),
            //     }

            //     let mut steps = Eval::new(e, &self.context);
            //     if let (Some(e), _continue) = steps.eval_last(100) {
            //         println!("→ ...");
            //         match &self.display_style {
            //             Style::LazyK => println!("→ {}", LazyKStyle(&e)),
            //             Style::ECMAScript => println!("→ {}", ECMAScriptStyle(&e)),
            //             _ => unreachable!(),
            //         }
            //     } else {
            //         // TODO
            //     }
            // }
            Command::Search(id) => self.context.get(&id).map_or(
                Output::Search {
                    input: id.clone(),
                    result: None,
                },
                |f| Output::Search {
                    input: id.clone(),
                    result: Some(f.clone()),
                },
            ),

            Command::Global => Output::Global {
                result: self.context.clone(),
            },

            Command::Unlambda(e) => Output::Unlambda {
                input: e.clone(),
                result: unlambda_shallow(e),
            },

            _ => panic!("not implemented"),
        }
    }
}
