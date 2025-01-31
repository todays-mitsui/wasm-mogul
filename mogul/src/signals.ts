import { makePersisted } from "@solid-primitives/storage";
import { type Accessor, createSignal, batch } from "solid-js";
import {
  type Context,
  type Aliases,
  type DisplayStyle,
  type Expr,
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
  const HISTORY_SIZE = 100;
  setCommandHistory((prev) => [...prev, command].slice(-HISTORY_SIZE));
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

export function resetContext() {
  setContext(defaultContext());
}

export function clearContext() {
  setContext({});
}

export const [aliases, setAliases] = createSignal<Aliases>({});

function createToolBoxSignal() {
  const [isOpen, setIsOpen] = createSignal(false);
  return {
    isOpen,
    open() {
      setIsOpen(true);
    },
    close() {
      setIsOpen(false);
    },
    toggle() {
      setIsOpen((state) => !state);
    },
  };
}

const sideToolSignals = {
  context: createToolBoxSignal(),
  settings: createToolBoxSignal(),
};

export type SideTools = keyof typeof sideToolSignals;

export const sideTools = {
  isOpen(name: SideTools) {
    return sideToolSignals[name].isOpen();
  },
  open(name: SideTools) {
    batch(() => {
      for (const toolBox of Object.values(sideToolSignals)) {
        toolBox.close();
      }
      sideToolSignals[name].open();
    });
  },
  toggle(name: SideTools) {
    const isOpen = sideToolSignals[name].isOpen();
    batch(() => {
      for (const toolBox of Object.values(sideToolSignals)) {
        toolBox.close();
      }
      if (!isOpen) {
        sideToolSignals[name].open();
      }
    });
  },
  closeAll() {
    batch(() => {
      for (const toolBox of Object.values(sideToolSignals)) {
        toolBox.close();
      }
    });
  },
};

export const [consoleOut, setConsoleOut] = createSignal<ConsoleItem[]>([]);

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

export interface ConsoleItemUnlambda {
  type: "Unlambda";
  expr: Expr;
  result: Expr;
}

export interface ConsoleItemParseError {
  type: "ParseError";
  message: string;
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
  | ConsoleItemContext
  | ConsoleItemUnlambda
  | ConsoleItemParseError;
