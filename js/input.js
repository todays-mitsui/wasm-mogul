import { run } from './run.js';

/**
 * @param {object} module
 */
export async function initInput(module) {
  const input = document.querySelector('#input input');
  const submit = document.querySelector('#input button[type=submit]');

  submit.disabled = !input.value.trim();
  input.addEventListener('input', function (event) {
    submit.disabled = !input.value.trim();
  });

  input.addEventListener('focusin', function (event) {
    input.placeholder = '';
  });
  input.addEventListener('focusout', function (event) {
    input.placeholder = '_';
  });

  if (location.search) {
    const param = new URLSearchParams(location.search);
    if (param.has('run')) {
      const srcs = param.getAll('run');

      const outputBox = document.querySelector('#output');

      await Promise.allSettled(srcs.map(src => {
        console.info({ run: src });
        return run(module, src, outputBox);
      }));
    }

    if (param.has('input')) {
      input.value = param.get('input');
    }
  }
}
