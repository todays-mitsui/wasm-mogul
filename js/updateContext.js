const contextBox = document.querySelector('#functions .inner ul');

/**
 * @param {string[]} context
 * @returns {void}
 */
export function updateContext(context) {
  while (contextBox.firstChild) {
    contextBox.removeChild(contextBox.firstChild);
  }

  for (const row of context) {
    const code = document.createElement('code');
    code.textContent = row;

    const li = document.createElement('li');
    li.appendChild(code);

    contextBox.appendChild(li);
  }
}
