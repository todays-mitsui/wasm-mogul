/* tslint:disable */
/* eslint-disable */
export function parseCommand(input: string): Command;
export function parseExpr(input: string): Expr;
export function renderExpr(expr: Expr, displayStyle: DisplayStyle): string;
export function expand(context: Context, expr: Expr): Expr;
export function unlambdaRecursive(context: Context, expr: Expr): Expr;
export function unlambdaRecursive_(context: Context, expr: Expr): Expr;
export function unlambdaIota(context: Context, expr: Expr): Expr;
export function renderFunc(func: Func, displayStyle: DisplayStyle): string;
export function defaultContext(): Context;
export type Command = { type: "Delete"; identifier: string } | { type: "Update"; func: Func } | { type: "Reduce"; expr: Expr } | { type: "ReduceLast"; expr: Expr } | { type: "ReduceHead"; count: number; expr: Expr } | { type: "ReduceTail"; count: number; expr: Expr } | { type: "Query"; identifier: string } | { type: "Context" } | { type: "Unlambda"; level: number; expr: Expr };

export interface IteratorResult {
    done: boolean;
    value: ReduceResult | null;
}

export type ExprRange = { start: number; end: number };

export interface FormedExpr {
    expr: string;
    reducibleRange: ReducibleRange | null;
}

export interface ReduceResult {
    step: number;
    expr: Expr;
    formed: FormedReducedExpr;
}

export interface FormedReducedExpr {
    expr: string;
    reducedRange: ExprRange;
    reducibleRange: ReducibleRange | null;
}

export interface ReducibleRange {
    entire: ExprRange;
    callee: ExprRange;
    args: ExprRange[];
}

export type Expr = { Variable: { identifier: string } } | { Symbol: { identifier: string } } | { Apply: { lhs: Expr; rhs: Expr } } | { Lambda: { param: string; body: Expr } };

export type Aliases = Record<Identifier, Expr>;

export type DisplayStyle = "EcmaScript" | "LazyK";

export interface Func {
    name: Identifier;
    params: Identifier[];
    body: Expr;
}

export type Context = Record<Identifier, Func>;

export type Identifier = string;

export class Reducer {
  free(): void;
  constructor(context: Context, aliases: Aliases, expr: Expr, displayStyle?: DisplayStyle | null);
  next(): IteratorResult;
  displayStyle: DisplayStyle;
  readonly formed: FormedExpr;
  readonly hasNext: boolean;
}
