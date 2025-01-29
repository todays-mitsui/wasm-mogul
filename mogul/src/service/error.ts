import { putConsoleItem } from "~/service/console";

export function showError(error: Error) {
  putConsoleItem({ type: "ParseError", message: error.message });
}
