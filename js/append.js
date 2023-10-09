export function appendOl(container, contents, formatter = null, start = 0) {
  const ol = document.createElement('ol');
  ol.setAttribute('start', start);
  container.appendChild(ol);

  for (const content of contents) {
    const li = document.createElement('li');
    li.textContent = formatter ? formatter(content) : content;
    ol.appendChild(li);
  }
}

export function appendUl(container, contents, formatter = null, start = 0) {
  const ol = document.createElement('ul');
  ol.setAttribute('start', start);
  container.appendChild(ol);

  for (const content of contents) {
    const li = document.createElement('li');
    li.textContent = formatter ? formatter(content) : content;
    ol.appendChild(li);
  }
}
