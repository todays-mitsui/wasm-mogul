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
  const { expr, steps } = module.lambda_calculus(src, 'ECMAScript');
  console.log({ src, expr, steps });

  const ol = document.createElement('ol');
  ol.setAttribute('start', '0');

  container.appendChild(ol);

  const li = document.createElement('li');
  li.textContent = expr;
  ol.appendChild(li);

  for (const step of steps) {
    const li = document.createElement('li');
    li.textContent = step;
    ol.appendChild(li);
  }

  container.scrollTo({
    top: container.scrollHeight,
    behavior: 'smooth',
  });
}

document.addEventListener('DOMContentLoaded', main);
