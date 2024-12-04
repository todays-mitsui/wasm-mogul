import {
  type DisplayStyle,
  type Func,
  renderFunc as render,
} from "../../../ski3/pkg/index";
import { displayStyle as getDisplayStyle } from "~/signals";
export { type DisplayStyle, type Func };

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
