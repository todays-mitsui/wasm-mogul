import { initDetails } from './details.js';
import { implant } from './implant.js';
import { initInput } from './input.js';
import { initRandomSpell } from './randomSpell.js';
import { run } from './run.js';
import { initSettings } from './settings.js';
import { updateContext } from './updateContext.js';
import { highlightReduced } from './highlight.js';

async function main() {
  console.info('ski Mogul, version 0.2.0');

  initDetails();
  initRandomSpell();

  const module = await import('../ski/pkg/index.js');

  updateContext(module);
  initSettings(module);
  initInput(module);

  const form = document.querySelector('#input form');
  form.addEventListener('submit', function (event) {
    event.preventDefault();
    onSubmit(module);
  });

  implant(module);

  document.body.addEventListener('mouseover', event => {
    const elem = event.target;

    if (
      !(elem instanceof HTMLLIElement)
      || !(elem.parentElement instanceof HTMLOListElement)
      || !elem.parentElement.classList.contains('eval')
    ) { return; }

    const unhighlight = highlightReduced(elem);
    if (!unhighlight) { return; }

    const onMouseLeave = () => {
      unhighlight();
      elem.removeEventListener('mouseleave', onMouseLeave);
    };
    elem.addEventListener('mouseleave', onMouseLeave);
  });
}

/**
 * @param {{ execute: (src: string, style: 'ECMAScript' | 'Lazy_K') => { expr: string; steps: string[]; } }} module
 */
async function onSubmit(module) {
  const input = document.querySelector('#input input');
  const outputBox = document.querySelector('#output');

  const src = input.value;
  if (!src.trim()) { return; }  // 何も入力されていないなら何もしない

  input.value = '';

  await run(module, src, outputBox);

  outputBox.scrollTo({
    top: outputBox.scrollHeight,
    behavior: 'smooth',
  });

  input.focus();
  input.dispatchEvent(new Event('input'));
}

main();
