import { initDetails } from './details.js';
import { displayEvalInit, displayEval, displayUpdate, displayDelete, displayUnlambda, displayCodeList, displayParseError } from './display.js';
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

  const module = await import('../ski/pkg/index.js');

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

  let command;
  try {
    command = Command.parse(src);
  }
  catch (err) {
    console.info({ err });
    displayParseError(src);
    hideLoader();
    input.focus();
    input.dispatchEvent(new Event('input'));
    return;
  }


  const commandStr = command.toString();
  const run = execute(context, command, displayStyle);

  console.log({ command: commandStr, type: run.commandType });

  switch (run.commandType) {
    case 'del': {
      const id = run.input;
      const context = run.delResult;
      console.info({ id, context: context.getAll().map(func => func.format(displayStyle)) });
      displayDelete(id);
      updateContext(module);
    } break;

    case 'update': {
      const func = run.input;
      const context = run.updateResult;
      console.info({ func, context: context.getAll().map(func => func.format(displayStyle)) });
      displayUpdate(func);
      updateContext(module);
    } break;

    case 'eval': {
      const input = run.input;
      const result = run.evalResult;
      console.info({ input, iterator: result });
      const box = displayEvalInit(input);
      let done = false;
      while (!done) {
        await new Promise(resolve => setTimeout(resolve, 0));
        const next = result.next(displayStyle);
        done = next.done;
        if (next.value) {
          const { expr, step } = next.value;
          displayEval(box, expr);
          if (step >= STEP_LIMIT) {
            break;
          }
          if (step % 100 === 0) {
            outputBox.scrollTo({
              top: outputBox.scrollHeight,
              behavior: 'smooth',
            });
          }
        }
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
      console.info({ context: context.getAll().map(func => func.format(displayStyle)) });
      displayCodeList(context.getAll().map(func => func.format(displayStyle)));
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
