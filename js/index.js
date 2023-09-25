import("../pkg/index.js")
  .then(module => {
    console.log({ parse: module.parse, unlambda: module.unlambda });
    globalThis.parse = module.parse;
    globalThis.unlambda = module.unlambda;
  })
  .catch(console.error);
