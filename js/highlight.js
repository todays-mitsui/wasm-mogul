/**
 * @param {HTMLElement} elem
 */
export function highlightReduced(elem) {
  const rangesStr = elem.dataset.reduced;
  if (!rangesStr) { return; }
  const ranges = parseRanges(rangesStr);

  const code = elem.querySelector('code');
  const rawCode = code.textContent;

  const nodes = wrapParts(wrap, rawCode, ranges);
  const fragment = document.createDocumentFragment();
  for (const node of nodes) {
    fragment.appendChild(node);
  }

  code.innerHTML = '';
  code.appendChild(fragment);

  return () => {
    code.innerHTML = '';
    code.textContent = rawCode;
  };
}

/**
 * @param {string} textContent
 * @returns HTMLSpanElement
 */
function wrap(textContent) {
  const span = document.createElement('span');
  span.classList.add('reduced');
  span.textContent = textContent;
  return span;
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
 * @param {(textContent: string) => HTMLElement} wrap
 * @param {string} str
 * @param {Array<[number, number]>} ranges
 * @returns {Array<Text|HTMLElement>}
 */
function wrapParts(wrap, str, ranges) {
  ranges = ranges.toSorted(([aStart, _aEnd], [bStart, _bEnd]) => aStart - bStart);
  const strs = strSplits(str, ranges.flat());
  const nodes = [];
  for (let i = 0; i < strs.length; i++) {
    const s = strs[i];
    if (i % 2 === 0) {
      if (s !== "") {
        nodes.push(document.createTextNode(s));
      }
    } else {
      nodes.push(wrap(s));
    }
  }
  return nodes;
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
