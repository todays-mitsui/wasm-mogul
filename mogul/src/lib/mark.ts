import { type ExprRange, type ReducibleRange } from "../../../ski3/pkg/index";

export function markReducible(
  rawCode: string,
  ranges: ReducibleRange,
): DocumentFragment {
  const [before, subCode, after] = strSplits(rawCode, [
    ranges.entire.start,
    ranges.entire.end,
  ]);

  const parts = wrapParts(
    (index, text) => wrap(text, index === 0 ? "callee" : "argument"),
    subCode,
    [ranges.callee, ...ranges.args],
    before.length,
  );

  const span = document.createElement("span");
  span.classList.add("reducible");
  span.appendChild(parts);

  const fragment = document.createDocumentFragment();
  fragment.appendChild(document.createTextNode(before));
  fragment.appendChild(span);
  fragment.appendChild(document.createTextNode(after));

  return fragment;
}

export function markReduced(
  rawCode: string,
  ranges: readonly ExprRange[],
): DocumentFragment {
  return wrapParts((_index, text) => wrap(text, "reduced"), rawCode, ranges);
}

function wrap(textContent: string, className: string): HTMLSpanElement {
  const span = document.createElement("span");
  span.classList.add(className);
  span.textContent = textContent;
  return span;
}

function wrapParts(
  wrap: (index: number, textContent: string) => HTMLElement,
  str: string,
  ranges: readonly ExprRange[],
  offset = 0,
): DocumentFragment {
  ranges = ranges.toSorted((a, b) => a.start - b.start);
  const strs = strSplits(
    str,
    ranges
      .flatMap(({ start, end }) => [start, end])
      .map((index) => index - offset),
  );

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

function strSplits(str: string, indexes: readonly number[]): string[] {
  const strs: string[] = [];
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
