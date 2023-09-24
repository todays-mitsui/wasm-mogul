import("../pkg/index.js")
  .then(module => {
    console.log({ parse: module.parse });
    globalThis.parse = module.parse;
  })
  .catch(console.error);
