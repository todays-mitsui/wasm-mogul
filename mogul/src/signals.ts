import { makePersisted } from "@solid-primitives/storage";
import { type Accessor, createSignal } from "solid-js";
import {
  type Context,
  type DisplayStyle,
  type FormedExpr,
  type FormedReducedExpr,
  type Func,
  type Identifier,
  defaultContext,
} from "../../ski3/pkg/index";

export const [commandStr, setCommandStr] = createSignal("");

const [commandHistory, setCommandHistory] = makePersisted(
  createSignal<string[]>([]),
  {
    name: "command-history",
    storage: localStorage,
  },
);
export { commandHistory };
export function addCommandHistory(command: string) {
  setCommandHistory((prev) => [...prev, command].slice(-10));
}

export const [displayStyle, setDisplayStyle] = makePersisted(
  createSignal<DisplayStyle>("EcmaScript"),
  {
    name: "display-style",
    storage: localStorage,
  },
);

export const [context, setContext] = makePersisted(
  createSignal<Context>(defaultContext()),
  {
    name: "context",
    storage: sessionStorage,
  },
);

export const [console, setConsole] = createSignal<ConsoleItem[]>([]);

export interface ConsoleItemUpdate {
  type: "Update";
  func: Func;
}

export interface ConsoleItemDelete {
  type: "Delete";
  identifier: Identifier;
}

export interface ConsoleItemReduce {
  type: "Reduce";
  formed: FormedExpr;
  reduceResults: Accessor<
    {
      readonly step: number;
      readonly formed: FormedReducedExpr;
    }[]
  >;
}

export interface ConsoleItemReduceLast {
  type: "ReduceLast";
  formed: FormedExpr;
  reduceResult: Accessor<{
    readonly step: number;
    readonly formed: FormedReducedExpr;
  } | null>;
}

export interface ConsoleItemReduceHead {
  type: "ReduceHead";
  formed: FormedExpr;
  reduceResults: Accessor<
    {
      readonly step: number;
      readonly formed: FormedReducedExpr;
    }[]
  >;
}

export interface ConsoleItemReduceTail {
  type: "ReduceTail";
  formed: FormedExpr;
  reduceResults: Accessor<
    {
      readonly step: number;
      readonly formed: FormedReducedExpr;
    }[]
  >;
}

export interface ConsoleItemQueryDefined {
  type: "QueryDefined";
  func: Func;
}

export interface ConsoleItemQueryUndefined {
  type: "QueryUndefined";
  identifier: Identifier;
}

export interface ConsoleItemContext {
  type: "Context";
}

export type ConsoleItem =
  | ConsoleItemUpdate
  | ConsoleItemDelete
  | ConsoleItemReduce
  | ConsoleItemReduceLast
  | ConsoleItemReduceHead
  | ConsoleItemReduceTail
  | ConsoleItemQueryDefined
  | ConsoleItemQueryUndefined
  | ConsoleItemContext;
