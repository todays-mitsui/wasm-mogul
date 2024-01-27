/**
 * @param {Event} event
 * @returns {void}
 */
export function onMouseOver(event) {
  const hovered = event.target;

  if (
    !(hovered instanceof HTMLLIElement)
    || !(hovered.parentElement instanceof HTMLOListElement)
    || !hovered.parentElement.classList.contains('eval')
  ) { return; }

  const sibling = hovered.nextElementSibling;
  if (!(sibling instanceof HTMLLIElement)) { return; }

  highlight(hovered, sibling);
}

/**
 * @param {HTMLLIElement} hovered
 * @param {HTMLLIElement} sibling
 * @returns {void}
 */
function highlight(hovered, sibling) {
  const unHighlightNext = highlightNext(hovered);
  const unHighlightReduced = highlightReduced(sibling);

  const onMouseLeave = () => {
    if (unHighlightNext && typeof unHighlightNext === 'function') {
      unHighlightNext();
    }
    if (unHighlightReduced && typeof unHighlightReduced === 'function') {
      unHighlightReduced();
    }
    hovered.removeEventListener('mouseleave', onMouseLeave);
  };

  hovered.addEventListener('mouseleave', onMouseLeave);
}

/**
 * @param {HTMLElement} elem
 * @returns {() => void}
 */
function highlightNext(elem) {
  const rangesStr = elem.dataset.next;
  if (!rangesStr) { return; }

  const ranges = parseRanges(rangesStr);
  const entire = ranges.shift();

  const code = elem.querySelector('code');
  const rawCode = code.textContent;

  const [before, subCode, after] = strSplits(rawCode, entire);

  const parts = wrapParts(
    (index, text) => wrap(text, index === 0 ? 'callee' : 'argument'),
    subCode,
    ranges,
    before.length,
  );

  const span = document.createElement('span');
  span.classList.add('next');
  span.appendChild(parts);

  const fragment = document.createDocumentFragment();
  fragment.appendChild(document.createTextNode(before));
  fragment.appendChild(span);
  fragment.appendChild(document.createTextNode(after))

  console.log({ before, next: span.innerHTML, after });

  code.innerHTML = '';
  code.appendChild(fragment);

  return () => {
    code.innerHTML = '';
    code.textContent = rawCode;
  };
}

/**
 * @param {HTMLElement} elem
 * @returns {() => void}
 */
function highlightReduced(elem) {
  const rangesStr = elem.dataset.reduced;
  if (!rangesStr) { return; }

  const ranges = parseRanges(rangesStr);

  const code = elem.querySelector('code');
  const rawCode = code.textContent;

  const parts = wrapParts(
    (_index, text) => wrap(text, 'reduced'),
    rawCode,
    ranges,
  );

  code.innerHTML = '';
  code.appendChild(parts);

  return () => {
    code.innerHTML = '';
    code.textContent = rawCode;
  };
}

/**
 * @param {string} rangesStr
 * @returns {Array<[number, number]>}
 */
function parseRanges(rangesStr) {
  const ranges = [];
  for (const range of rangesStr.split(';')) {
    const [start, end] = range.split(',');
    ranges.push([parseInt(start), parseInt(end)]);
  }
  return ranges;
}

/**
 * @param {string} textContent
 * @param {string} className
 * @returns HTMLSpanElement
 */
function wrap(textContent, className) {
  const span = document.createElement('span');
  span.classList.add(className);
  span.textContent = textContent;
  return span;
}

/**
 * @param {(index: number, textContent: string) => HTMLElement} wrap
 * @param {string} str
 * @param {Array<[number, number]>} ranges
 * @param {number} offset
 * @returns {DocumentFragment}
 */
function wrapParts(wrap, str, ranges, offset = 0) {
  ranges = ranges.toSorted(([aStart, _aEnd], [bStart, _bEnd]) => aStart - bStart);
  const strs = strSplits(str, ranges.flat().map(index => index - offset));

  const fragment = document.createDocumentFragment();
  for (let i = 0; i < strs.length; i++) {
    const s = strs[i];
    if (i % 2 === 0) {
      if (s !== "") {
        fragment.appendChild(document.createTextNode(s));
      }
    } else {
      const index = (i - 1) * 0.5;
      fragment.appendChild(wrap(index, s));
    }
  }

  return fragment;
}

/**
 * @param {string} str
 * @param {Array<number>} indexes
 * @returns {Array<string>}
 */
function strSplits(str, indexes) {
  const strs = [];
  let current = 0;
  for (const index of indexes) {
    const [before, after] = [
      str.substring(0, index - current),
      str.substring(index - current),
    ];
    strs.push(before);
    str = after;
    current = index;
  }
  strs.push(str);
  return strs;
}
