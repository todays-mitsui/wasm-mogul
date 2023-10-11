export function appendOl(container, contents, formatter = null, start = 0) {
  const ol = document.createElement('ol');
  ol.setAttribute('start', start);
  container.appendChild(ol);

  for (const content of contents) {
    const li = document.createElement('li');
    ol.appendChild(li);

    const code = document.createElement('code');
    li.appendChild(code);
    code.textContent = formatter ? formatter(content) : content;
  }
}

export function appendUl(container, contents, formatter = null, start = 0) {
  const ul = document.createElement('ul');
  ul.setAttribute('start', start);
  container.appendChild(ul);

  for (const content of contents) {
    const li = document.createElement('li');
    ul.appendChild(li);

    const code = document.createElement('code');
    li.appendChild(code);
    code.textContent = formatter ? formatter(content) : content;
  }
}
