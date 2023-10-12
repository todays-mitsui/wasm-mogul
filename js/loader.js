const outputBox = document.querySelector('#output');
const submit = document.querySelector('#input button[type=submit]');

// let frame = 0;
// const frames = [
//   'Running.',
//   'Running..',
//   'Running...',
//   'Running....',
//   'Running.....',
//   'Running......',
// ];

// let intervalID = null;

export function showLoader() {
  const ul = document.createElement('ul');
  ul.classList.add('loader');
  outputBox.appendChild(ul);

  const li = document.createElement('li');
  const code = document.createElement('code');
  code.textContent = 'Running...';
  li.appendChild(code);
  ul.appendChild(li);

  // intervalID = setInterval(() => animate(code), 100);

  submit.disabled = true;
  submit.textContent = 'Running...';
}

export function hideLoader() {
  document.querySelector('#output .loader').remove();

  // if (intervalID != null) { clearInterval(intervalID); }

  submit.textContent = 'Run';
}

// /**
//  * @param {HTMLElement} elem
//  */
// function animate(elem) {
//   elem.textContent = frames[frame];
//   frame = (frame + 1) % frames.length;
// }
