import("../pkg/index.js")
  .then(module => {
    console.log({
      lambda_calculus: module.lambda_calculus,
      parse: module.parse,
      unlambda: module.unlambda,
    });
    globalThis.lambda_calculus = module.lambda_calculus;
    globalThis.parse = module.parse;
    globalThis.unlambda = module.unlambda;
  })
  .catch(console.error);
