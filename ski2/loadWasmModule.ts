export async function loadWasmModule(): Promise<WasmExports> {
  return await import('./pkg/index.js') as unknown as WasmExports;
}

export interface WasmExports {
  parseExpr(input: string): Expr;
  formatExpr(expr: Expr, displayStyle: DisplayStyle): string;
  parseCommand(input: string): Command;
	makeReducer(context: Context, expr: Expr, displayStyle: DisplayStyle): Reducer;
}

interface Context {
	[funcName: string]: Func;
}

interface Reducer extends Iterable<ReduceResult> {
	reducibleRange: string | null;
}

interface ReduceResult {
	step: number;
	expr: Expr;
	reducedRange: string;
	reducibleRange: string | null;
}

export type DisplayStyle = 'ECMAScript' | 'Lazy_K';

export type Command =
	| {
			type: "del";
			identifier: string;
	  }
	| {
			type: "update";
			func: Func;
	  }
	| {
			type: "evaluate";
			expr: Expr;
	  }
	| {
			type: "evaluateLast";
			expr: Expr;
	  }
	| {
			type: "evaluateHead";
			count: number;
			expr: Expr;
	  }
	| {
			type: "evaluateTail";
			count: number;
			expr: Expr;
	  }
	| {
			type: "query";
			identifier: string;
	  }
	| {
			type: "context";
	  }
	| {
			type: "unlambda";
			level: number;
			expr: Expr;
	  };

export interface Func {
  name: string;
  params: string[];
  body: Expr;
}

export type Expr =
	| {
			type: "variable";
			identifier: string;
	  }
	| {
			type: "symbol";
			identifier: string;
	  }
	| {
			type: "apply";
			func: Expr;
			arg: Expr;
	  }
	| {
			type: "lambda";
			param: string;
			body: Expr;
	  };
