import { context, setContext } from "~/signals";
import { type Func, defaultContext } from "../../../ski3/pkg/index";

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

export function resetContext() {
  setContext(defaultContext());
}

export function clearContext() {
  setContext({});
}
