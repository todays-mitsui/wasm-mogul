import { type Command, parseCommand } from "../../../ski3/pkg/index";
import {
  updateFunction,
  deleteFunction,
  queryFunction,
} from "~/service/context";
import { putConsoleItem } from "~/service/console";
export { Command, parseCommand };

export function runCommand(command: Command) {
  switch (command.type) {
    case "Delete":
      deleteFunction(command.identifier);
      putConsoleItem({ type: "Delete", identifier: command.identifier });
      return;
    case "Update":
      updateFunction(command.func);
      putConsoleItem({ type: "Update", func: command.func });
      return;
    case "Reduce":
      break;
    case "ReduceLast":
      break;
    case "ReduceHead":
      break;
    case "ReduceTail":
      break;
    case "Query":
      const func = queryFunction(command.identifier);
      if (func == null) {
        putConsoleItem({
          type: "QueryUndefined",
          identifier: command.identifier,
        });
        return;
      } else {
        putConsoleItem({ type: "QueryDefined", func });
        return;
      }
    case "Context":
      putConsoleItem({ type: "Context" });
      return;
    case "Unlambda":
      break;
  }
}
