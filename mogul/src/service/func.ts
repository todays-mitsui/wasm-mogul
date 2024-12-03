import { type Func, renderFunc as render } from "../../../ski3/pkg/index";
import { displayStyle } from "~/signals";

export function renderFunc(func: Func): [string, string] {
  const rendered = render(func, displayStyle());
  const i = rendered.indexOf("=");
  if (i < 0) {
    throw new Error("renderFunc: no '=' found in rendered function");
  }
  return [rendered.slice(0, i).trim(), rendered.slice(i + 1).trim()];
}
