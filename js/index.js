import("../pkg/index.js")
  .then(module => {
    console.log({
      eval_: module.eval_,
      parse: module.parse,
      unlambda: module.unlambda,
    });
    globalThis.eval_ = module.eval_;
    globalThis.parse = module.parse;
    globalThis.unlambda = module.unlambda;
  })
  .catch(console.error);
