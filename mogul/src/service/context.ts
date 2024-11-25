import { context, setContext } from "~/signals";
import { type Func } from "../../../ski3/pkg/index";
export { type Context } from "../../../ski3/pkg/index";

export function updateFunction(func: Func) {
  setContext((prev) => ({ ...prev, [func.name]: func }));
}

export function deleteFunction(identifier: string) {
  setContext((prev) => {
    const next = { ...prev };
    delete next[identifier];
    return next;
  });
}

export function queryFunction(identifier: string): Func | null {
  return context()[identifier] ?? null;
}
