import { createSignal } from "solid-js";
import { putConsoleItem } from "~/service/console";
import {
  deleteFunction,
  queryFunction,
  updateFunction,
} from "~/service/context";
import {
  type FormedReducedExpr,
  reduceHead,
  reduceLast,
  reduceTail,
} from "~/service/reduce";
import { unlambda } from "~/service/unlambda";
import { type Command, parseCommand } from "../../../ski3/pkg/index";
export { type Command, parseCommand };

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
      await reduceHead(command.expr, {
        onInit: ({ formed }) => {
          putConsoleItem({
            type: "Reduce",
            formed,
            reduceResults,
          });
        },
        onReduce: ({ step, formed }) => {
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
        onInit: ({ formed }) => {
          putConsoleItem({
            type: "ReduceLast",
            formed,
            reduceResult,
          });
        },
        onReduce: ({ step, formed }) => {
          setReduceResult({
            step,
            formed,
          });
        },
      });
      return;
    }

    case "ReduceHead": {
      const [reduceResults, setReduceResults] = createSignal<
        {
          readonly step: number;
          readonly formed: FormedReducedExpr;
        }[]
      >([]);
      await reduceHead(command.expr, {
        maxSteps: command.count,
        onInit: ({ formed }) => {
          putConsoleItem({
            type: "ReduceHead",
            formed,
            reduceResults,
          });
        },
        onReduce: ({ step, formed }) => {
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

    case "ReduceTail": {
      const [reduceResults, setReduceResults] = createSignal<
        {
          readonly step: number;
          readonly formed: FormedReducedExpr;
        }[]
      >([]);
      await reduceTail(command.expr, {
        count: command.count,
        onInit: ({ formed }) => {
          putConsoleItem({
            type: "ReduceTail",
            formed,
            reduceResults,
          });
        },
        onReduce: ({ step, formed }) => {
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
      const result = unlambda(command.level, command.expr);
      putConsoleItem({ type: "Unlambda", expr: command.expr, result });
      return;
    }
  }
}
