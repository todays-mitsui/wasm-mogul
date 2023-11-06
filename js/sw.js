import { execute } from '../ski/pkg/index.js';

console.log({ execute });

self.addEventListener('message', async (event) => {
  const { src } = event.data;
  const output = execute(src);
  event.source.postMessage({ output });
});
