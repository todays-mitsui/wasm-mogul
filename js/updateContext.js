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

  for (const func of context.getAll()) {
    const code = document.createElement('code');
    code.textContent = func.format(displayStyle);

    const li = document.createElement('li');
    li.appendChild(code);

    contextBox.appendChild(li);
  }
}
