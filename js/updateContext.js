const contextBox = document.querySelector('#context .inner ul');

/**
 * @param {Context} Context
 * @param {() => DisplayStyle} getDisplayStyle
 * @returns {void}
 */
export function updateContext({ Context, getDisplayStyle }) {
  while (contextBox.firstChild) {
    contextBox.removeChild(contextBox.firstChild);
  }

  const displayStyle = getDisplayStyle();
  const context = new Context();

  for (const row of context.getAll(displayStyle)) {
    const code = document.createElement('code');
    code.textContent = row;

    const li = document.createElement('li');
    li.appendChild(code);

    contextBox.appendChild(li);
  }
}
