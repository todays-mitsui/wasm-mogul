import { appendOl, appendUl } from './append.js';

async function main() {
  const form = document.getElementById('form');
  const input = document.getElementById('src');

  const module = await import('../pkg/index.js');

  console.log('ready to calculate');
  console.log({ lambda_calculus: module.lambda_calculus });

  form.addEventListener('submit', function (event) {
    event.preventDefault();
    onSubmit(module, input, container);
  });
}

/**
 * @param {{ lambda_calculus: (src: string, style: 'ECMAScript' | 'Lazy_K') => { expr: string; steps: string[]; } }} module
 * @param {HTMLInputElement} input
 * @param {HTMLDivElement} container
 */
function onSubmit(module, input, container) {
  const src = input.value;
  if (!src.trim()) { return; }  // 何も入力されていないなら何もしない

  input.value = '';
  const output = module.lambda_calculus(src, 'ECMAScript');

  showOutput(container, output);

  container.scrollTo({
    top: container.scrollHeight,
    behavior: 'smooth',
  });
}

function showOutput(container, output) {
  console.log(output);

  switch (output.type) {
    case 'Del': {
      const { input: id, result: context } = output;
      console.log({ id, context });
      appendOl(container, [`${id} = ${id}`]);
    } break;

    case 'Update': {
      const { input: func } = output;
      appendOl(container, [func]);
    } break;

    case 'Eval': {
      const { input: expr, steps } = output;
      appendOl(container, [expr, ...steps.map(({ expr }) => expr)]);
    } break;

    case 'Search': {
      const { input: id, result: func } = output;
      appendOl(container, [func == null ? `${id} = ${id}` : func]);
    } break;

    case 'Global': {
      const { result: context } = output;
      console.log({ context });
      appendUl(container, context);
    } break;

    case 'Unlambda': {
      const { input, result } = output;
      appendOl(container, [`${input} ~ ${result}`]);
    } break;
  }
}

document.addEventListener('DOMContentLoaded', main);
