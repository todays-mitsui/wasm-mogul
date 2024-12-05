/**
 * @param {string} rawCode
 * @param {string} rangesStr
 * @returns {DocumentFragment}
 */
export function highlightNext(rawCode, rangesStr) {
  const ranges = parseRanges(rangesStr);
  const entire = ranges.shift();

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

  return fragment;
}

/**
 * @param {string} rawCode
 * @param {string} rangesStr
 * @returns {DocumentFragment}
 */
export function highlightReduced(rawCode, rangesStr) {
  const ranges = parseRanges(rangesStr);

  return wrapParts(
    (_index, text) => wrap(text, 'reduced'),
    rawCode,
    ranges,
  );
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
      const index = (i - 1) * 0.5; // i >> 1
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
