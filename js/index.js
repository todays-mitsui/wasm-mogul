import { initDetails } from './details.js';
import { displayEvalInit, displayEval, displayUpdate, displayDelete, displayUnlambda, displayCodeList } from './display.js';
import { implant } from './implant.js';
import { initInput } from './input.js';
import { showLoader, hideLoader } from './loader.js';
import { initRandomSpell } from './randomSpell.js';
import { initSettings } from './settings.js';
import { updateContext } from './updateContext.js';

const STEP_LIMIT = 2000;

async function main() {
  console.info('ski Mogul, version 0.2.0');

  initDetails();
  initInput();
  initRandomSpell();

  // const outputBox = document.querySelector('#output');
  // const input = document.querySelector('#input input');

  const module = await import('../ski/pkg/index.js');

  console.log(module);

  updateContext(module);
  initSettings(module);

  const form = document.querySelector('#input form');
  form.addEventListener('submit', function (event) {
    event.preventDefault();
    onSubmit(module);
  });

  implant(module);
}

/**
 * @param {{ execute: (src: string, style: 'ECMAScript' | 'Lazy_K') => { expr: string; steps: string[]; } }} module
 */
async function onSubmit(module) {
  const { Command, Context, getDisplayStyle, execute } = module;

  const input = document.querySelector('#input input');
  const outputBox = document.querySelector('#output');

  const src = input.value;
  if (!src.trim()) { return; }  // 何も入力されていないなら何もしない

  showLoader();

  input.value = '';

  const displayStyle = getDisplayStyle();
  const context = new Context();

  let command = Command.parse(src);

  console.log({ command: command.toString() });

  const run = execute(context, command);

  console.log({ 'runResult.type': run.type });

  switch (run.type) {
    case 'del': {
      const id = run.input;
      const context = run.delResult;
      console.info({ id, context: context.getAll(displayStyle) });
      displayDelete(id);
      updateContext(module);
    } break;

    case 'update': {
      const func = run.input;
      const context = run.updateResult;
      console.info({ func, context: context.getAll(displayStyle) });
      displayUpdate(func);
      updateContext(module);
    } break;

    case 'eval': {
      const input = run.input;
      const result = run.evalResult;
      console.info({ input, result });
      const box = displayEvalInit(input);
      let done = false;
      while (!done) {
        await new Promise(resolve => setTimeout(resolve, 0));
        const next = result.next();
        done = next.done;
        if (next.value) {
          const { expr, step } = next.value;
          displayEval(box, expr);
          if (step >= STEP_LIMIT) {
            break;
          }
        }
        outputBox.scrollTo({
          top: outputBox.scrollHeight,
          behavior: 'smooth',
        });
      }
    } break;

    case 'search': {
      const id = run.input;
      const func = run.searchResult;
      console.info({ id, func: func.format(displayStyle) });
      displayUpdate(func == null ? `${id} = ${id}` : func.format(displayStyle));
    } break;

    case 'context': {
      const context = run.contextResult;
      console.info({ context: context.getAll(displayStyle) });
      displayCodeList(context.getAll(displayStyle));
    } break;

    case 'unlambda': {
      const input = run.input;
      const result = run.unlambdaResult;
      const level = run.unlambdaLevel;
      console.info({ input, result, level });
      displayUnlambda(input, result.format(displayStyle));
    } break;
  }

  hideLoader();

  outputBox.scrollTo({
    top: outputBox.scrollHeight,
    behavior: 'smooth',
  });

  input.focus();
  input.dispatchEvent(new Event('input'));
}

main();
