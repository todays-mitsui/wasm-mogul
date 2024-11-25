import { type ConsoleItem, setConsole } from "~/signals";

export function putConsoleItem(item: ConsoleItem) {
  setConsole((prev) => [...prev, item]);
}
