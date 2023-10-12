import { initDetails } from './details.js';
import { displayEval, displayUpdate, displayDelete, displayUnlambda, displayCodeList } from './display.js';
import { implant } from './implant.js';
import { initInput } from './input.js';
import { initRandomSpell } from './randomSpell.js';
import { initSettings } from './settings.js';
import { updateContext } from './updateContext.js';

async function main() {
  initDetails();
  initInput();
  initRandomSpell();

  const outputBox = document.querySelector('#output');
  const form = document.querySelector('#input form');
  const input = document.querySelector('#input input');

  const module = await import('../pkg/index.js');

  updateContext(module.context());
  initSettings(module);

  form.addEventListener('submit', function (event) {
    event.preventDefault();
    onSubmit(module, input, outputBox);
  });

  implant(module);
}

/**
 * @param {{ execute: (src: string, style: 'ECMAScript' | 'Lazy_K') => { expr: string; steps: string[]; } }} module
 * @param {HTMLInputElement} input
 * @param {HTMLDivElement} outputBox
 */
function onSubmit(module, input, outputBox) {
  const src = input.value;
  if (!src.trim()) { return; }  // 何も入力されていないなら何もしない

  input.value = '';
  const output = module.execute(src, 'ECMAScript');

  showOutput(output);

  outputBox.scrollTo({
    top: outputBox.scrollHeight,
    behavior: 'smooth',
  });

  input.focus();
  input.dispatchEvent(new Event('input'));
}

function showOutput(output) {
  console.log({ result: output });

  switch (output.type) {
    case 'Del': {
      const { input: id, result: context } = output;
      console.log({ id, context });
      displayDelete(id);
      updateContext(context);
    } break;

    case 'Update': {
      const { input: func, result: context } = output;
      displayUpdate(func);
      updateContext(context);
    } break;

    case 'Eval': {
      const { input: expr, steps } = output;
      displayEval(expr, steps.map(({ expr }) => expr));
    } break;

    case 'Search': {
      const { input: id, result: func } = output;
      displayUpdate(func == null ? `${id} = ${id}` : func);
    } break;

    case 'Context': {
      const { result: context } = output;
      displayCodeList(context);
    } break;

    case 'Unlambda': {
      const { input, result } = output;
      displayUnlambda(input, result);
    } break;
  }
}

document.addEventListener('DOMContentLoaded', main);
