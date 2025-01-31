import { displayStyle as getDisplayStyle } from "~/signals";
import {
  type DisplayStyle,
  type Func,
  renderFunc as render,
} from "../../../ski3/pkg/index";
export type { DisplayStyle, Func };

export function renderFunc(
  func: Func,
  displayStyle?: DisplayStyle,
): [string, string] {
  const rendered = render(func, displayStyle ?? getDisplayStyle());
  const i = rendered.indexOf("=");
  if (i < 0) {
    throw new Error("renderFunc: no '=' found in rendered function");
  }
  return [rendered.slice(0, i).trim(), rendered.slice(i + 1).trim()];
}

// ========================================================================== //

export function sortFuncs(funcs: Func[]): Func[] {
  const taggedFuncs: [Func, Tag][] = funcs.map<[Func, Tag]>((func) => {
    const { n: name } = func;
    return [func, sortTag(name)];
  });

  taggedFuncs.sort(([_, a], [__, b]) => {
    const cmp = a.label.localeCompare(b.label);
    if (cmp !== 0) {
      return cmp;
    }
    if (a.index === null) {
      return b.index === null ? 0 : 1;
    }
    if (b.index === null) {
      return -1;
    }
    return a.index - b.index;
  });

  return taggedFuncs.map(([func]) => func);
}

interface Tag {
  label: string;
  index: number | null;
}

function sortTag(name: string): Tag {
  const m = name.match(/^(.*?)(\d+)?$/);
  if (!m) {
    throw new Error(`sortTag: invalid tag name: ${name}`);
  }
  return { label: m[1], index: m[2] ? parseInt(m[2], 10) : null };
}
