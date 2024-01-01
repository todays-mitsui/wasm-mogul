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
  initRandomSpell();

  const module = await import('../ski/pkg/index.js');

  updateContext(module);
  initSettings(module);

  const form = document.querySelector('#input form');
  form.addEventListener('submit', function (event) {
    event.preventDefault();
    onSubmit(module);
  });

  initInput();

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

  input.value = '';

  await run(module, src, getDisplayStyle(), outputBox);

  outputBox.scrollTo({
    top: outputBox.scrollHeight,
    behavior: 'smooth',
  });

  input.focus();
  input.dispatchEvent(new Event('input'));
}

/**
 * @param {object} module
 * @param {string} src
 * @param {'ECMAScript'|'Lazy_K'} displayStyle
 * @param {HTMLElement} outputBox
 * @returns
 */
async function run(module, src, displayStyle, outputBox) {
  const { Command, Context, execute } = module;

  showLoader();

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

  const context = new Context();
  const exec = execute(context, command, displayStyle);

  console.log({
    command: commandStr,
    type: exec.commandType
  });

  switch (exec.commandType) {
    case 'del': {
      const id = exec.input;
      const context = exec.delResult;
      console.info({ id, context: context.getAll().map(func => func.format(displayStyle)) });
      displayDelete(id);
      updateContext(module);
    } break;

    case 'update': {
      const func = exec.input;
      const context = exec.updateResult;
      console.info({ func, context: context.getAll().map(func => func.format(displayStyle)) });
      displayUpdate(func);
      updateContext(module);
    } break;

    case 'eval': {
      const input = exec.input;
      const result = exec.evalResult;
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
      const id = exec.input;
      const func = exec.searchResult;
      console.info({ id, func: func.format(displayStyle) });
      displayUpdate(func == null ? `${id} = ${id}` : func.format(displayStyle));
    } break;

    case 'context': {
      const context = exec.contextResult;
      console.info({ context: context.getAll().map(func => func.format(displayStyle)) });
      displayCodeList(context.getAll().map(func => func.format(displayStyle)));
    } break;

    case 'unlambda': {
      const input = exec.input;
      const result = exec.unlambdaResult;
      const level = exec.unlambdaLevel;
      console.info({ input, result, level });
      displayUnlambda(input, result.format(displayStyle));
    } break;
  }

  hideLoader();
}

main();
