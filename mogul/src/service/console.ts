import { type ConsoleItem, setConsoleOut } from "~/signals";

export function putConsoleItem(item: ConsoleItem) {
  setConsoleOut((prev) => [...prev, item]);
}
