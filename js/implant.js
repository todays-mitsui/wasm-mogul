export function implant(module) {
  const methodNames = [];
  for (const key of Object.keys(module)) {
    const methodName = `mogul${snakeToCamel(key)}`;

    if (
      !module.hasOwnProperty(key)
      || globalThis[methodName]
      || typeof module[key] !== 'function'
      || key.charAt(0) === '_'
    ) { continue; }

    Object.defineProperty(globalThis, methodName, {
      value: module[key],
      writable: false,
    });
    methodNames.push(methodName);
  }

  const methods = Object.create(null);
  for (const methodName of methodNames) {
    methods[methodName] = globalThis[methodName];
  }
  console.info(methods);
}

/**
 * スネークケースの文字列をキャメルケースに変換する
 *
 * @param {string} snakeCase
 * @returns {string}
 */
function snakeToCamel(snakeCase) {
  return snakeCase
    .replace(/_./g, s => s.charAt(1).toUpperCase())
    .replace(/^./, s => s.toUpperCase());
}
