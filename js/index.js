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

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      const li = document.createElement('li');
      li.textContent = `${id} = ${id}`;
      ol.appendChild(li);
    } break;

    case 'Update': {
      const { input: func } = output;

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      const li = document.createElement('li');
      li.textContent = func;
      ol.appendChild(li);
    } break;

    case 'Eval': {
      const { input: expr, steps } = output;

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      const li = document.createElement('li');
      li.textContent = expr;
      ol.appendChild(li);

      for (const { expr } of steps) {
        const li = document.createElement('li');
        li.textContent = expr;
        ol.appendChild(li);
      }
    } break;

    case 'Search': {
      const { input: id, result: func } = output;

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      const li = document.createElement('li');
      li.textContent = func == null
        ? `${id} = ${id}`
        : func;
      ol.appendChild(li);
    } break;

    case 'Global': {
      const { result: context } = output;
      console.log({ context });

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      for (const func of context) {
        const li = document.createElement('li');
        li.textContent = func;
        ol.appendChild(li);
      }
    } break;

    case 'Unlambda': {
      const { input, result } = output;

      const ol = document.createElement('ol');
      ol.setAttribute('start', '0');
      container.appendChild(ol);

      const li = document.createElement('li');
      li.textContent = `${input} == ${result}`;
      ol.appendChild(li);

    } break;
  }
}

document.addEventListener('DOMContentLoaded', main);
