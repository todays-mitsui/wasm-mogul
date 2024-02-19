import { displayEvalInit, displayEval, displayUpdate, displayDelete, displayUnlambda, displayCodeList, displayParseError } from './display.js';
import { showLoader, hideLoader } from './loader.js';
import { updateContext } from './updateContext.js';

const STEP_LIMIT = 2000;

/**
 * @param {object} module
 * @param {string} src
 * @param {'ECMAScript'|'Lazy_K'} displayStyle
 * @param {HTMLElement} outputBox
 * @returns
 */
export async function run(module, src, outputBox) {
  const { Command, Context, execute, getDisplayStyle } = module;

  showLoader();

  let command;
  try {
    command = Command.parse(src);
  }
  catch (err) {
    console.info({ err });
    displayParseError(src);
    hideLoader();
    return;
  }

  const commandStr = command.toString();

  const context = new Context();
  const displayStyle = getDisplayStyle();
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
      console.log(module);
      const func = exec.input;
      const context = exec.updateResult;
      console.info({ func, context: context.getAll().map(func => func.format(displayStyle)) });
      displayUpdate(func);
      updateContext(module);
    } break;

    case 'eval': {
      const input = exec.input;
      const inputNext = exec.evalInputNext;
      const result = exec.evalResult;
      console.info({ input, iterator: result });
      const box = displayEvalInit(input, inputNext);
      let done = false;
      while (!done) {
        await new Promise(resolve => setTimeout(resolve, 0));
        const next = result.next(displayStyle);
        done = next.done;
        if (next.value) {
          const { expr, step, reduced: reduced_range, next: next_range } = next.value;
          displayEval(box, expr, reduced_range, next_range);
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

    case 'query': {
      const id = exec.input;
      const func = exec.queryResult;
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
