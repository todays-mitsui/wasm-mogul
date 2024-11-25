import { type Command, parseCommand } from "../../../ski3/pkg/index";
import {
  updateFunction,
  deleteFunction,
  queryFunction,
} from "~/service/context";
import { putConsoleItem } from "~/service/console";
import { reduce, reduceLast, type FormedReducedExpr } from "~/service/reduce";
import { createSignal } from "solid-js";
export { Command, parseCommand };

export async function runCommand(command: Command) {
  switch (command.type) {
    case "Delete": {
      deleteFunction(command.identifier);
      putConsoleItem({ type: "Delete", identifier: command.identifier });
      return;
    }

    case "Update": {
      updateFunction(command.func);
      putConsoleItem({ type: "Update", func: command.func });
      return;
    }

    case "Reduce": {
      const [reduceResults, setReduceResults] = createSignal<
        {
          readonly step: number;
          readonly formed: FormedReducedExpr;
        }[]
      >([]);
      await reduce(command.expr, {
        onInit: ({ reducer }) => {
          putConsoleItem({
            type: "Reduce",
            formed: reducer.formed,
            reduceResults,
          });
        },
        onReduce: ({ reduceResult: { step, formed } }) => {
          setReduceResults((prev) => [
            ...prev,
            {
              step,
              formed,
            },
          ]);
        },
      });
      return;
    }

    case "ReduceLast": {
      const [reduceResult, setReduceResult] = createSignal<{
        readonly step: number;
        readonly formed: FormedReducedExpr;
      } | null>(null);
      await reduceLast(command.expr, {
        onInit: ({ reducer }) => {
          putConsoleItem({
            type: "ReduceLast",
            formed: reducer.formed,
            reduceResult: reduceResult,
          });
        },
        onReduce: ({ reduceResult: { step, formed } }) => {
          setReduceResult({
            step,
            formed,
          });
        },
      });
      return;
    }

    case "ReduceHead": {
      return;
    }

    case "ReduceTail": {
      return;
    }

    case "Query": {
      const func = queryFunction(command.identifier);
      if (func == null) {
        putConsoleItem({
          type: "QueryUndefined",
          identifier: command.identifier,
        });
      } else {
        putConsoleItem({ type: "QueryDefined", func });
      }
      return;
    }

    case "Context": {
      putConsoleItem({ type: "Context" });
      return;
    }

    case "Unlambda": {
      return;
    }
  }
}
