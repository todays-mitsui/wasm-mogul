const outputBox = document.querySelector('#output');

/**
 * @param {string} expr
 * @returns {HTMLOListElement}
 */
export function displayEvalInit(expr) {
  const ol = document.createElement('ol');
  ol.classList.add('eval');
  outputBox.appendChild(ol);

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = expr;
    li.appendChild(code);
    ol.appendChild(li);
  }

  return ol;
}

/**
 * @param {HTMLOListElement} ol
 * @param {string} expr
 * @returns {void}
 */
export function displayEval(ol, expr) {
  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = expr;
    li.appendChild(code);
    ol.appendChild(li);
  }
}

/**
 * @param {string} func
 * @returns {void}
 */
export function displayUpdate(func) {
  const ul = document.createElement('ul');
  ul.classList.add('update');
  outputBox.appendChild(ul);

  const matches = func.match(/^([^=]+)=(.+)$/);

  if (!matches) { return; }

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = matches[1].trim();
    li.appendChild(code);
    ul.appendChild(li);
  }

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = matches[2].trim();
    li.appendChild(code);
    ul.appendChild(li);
  }
}

/**
 * @param {string} id
 * @returns {void}
 */
export function displayDelete(id) {
  const ul = document.createElement('ul');
  ul.classList.add('update');
  outputBox.appendChild(ul);

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = id;
    li.appendChild(code);
    ul.appendChild(li);
  }

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = id;
    li.appendChild(code);
    ul.appendChild(li);
  }
}

/**
 * @param {string} input
 * @param {string} result
 * @returns {void}
 */
export function displayUnlambda(input, result) {
  const ul = document.createElement('ul');
  ul.classList.add('unlambda');
  outputBox.appendChild(ul);

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = input;
    li.appendChild(code);
    ul.appendChild(li);
  }

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = result;
    li.appendChild(code);
    ul.appendChild(li);
  }
}

/**
 * @param {string[]} exprs
 * @returns {void}
 */
export function displayCodeList(exprs) {
  const ul = document.createElement('ul');
  outputBox.appendChild(ul);

  for (const expr of exprs) {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = expr;
    li.appendChild(code);
    ul.appendChild(li);
  }
}

/**
 * @param {string} input
 * @returns {void}
 */
export function displayParseError(input) {
  const ul = document.createElement('ul');
  ul.classList.add('error');
  outputBox.appendChild(ul);

  {
    const li = document.createElement('li');
    const code = document.createElement('code');
    code.textContent = `Parse Error: "${input}"`;
    li.appendChild(code);
    ul.appendChild(li);
  }
}
