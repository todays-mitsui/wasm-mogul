import { context, aliases, displayStyle } from "~/signals";
import {
  type Expr,
  type FormedReducedExpr,
  type ReduceResult,
  Reducer,
} from "../../../ski3/pkg/index";
export type { ReduceResult, FormedReducedExpr };

const MAX_STEPS = 1000;

interface ReduceOptions {
  onInit?: (reducer: Reducer) => void;
  onReduce?: (reduceResult: ReduceResult) => void;
  onEnd?: (reduceResult: ReduceResult) => void;
  maxSteps?: number;
}

export async function reduceHead(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, onEnd, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), aliases(), expr, displayStyle());

  onInit?.(reducer);

  let reduceResult: ReduceResult | null = null;
  while (true) {
    const result = reducer.next();

    if (result.done) break;

    reduceResult = result.value;

    if (reduceResult == null) continue;

    onReduce?.(reduceResult);

    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) break;

    await new Promise((resolve) => setTimeout(resolve, 0));
  }

  reduceResult && onEnd?.(reduceResult);
}

export async function reduceTail(
  expr: Expr,
  options: ReduceOptions & { count: number },
): Promise<void> {
  const { onInit, onReduce, onEnd, maxSteps, count } = options;

  const reducer = new Reducer(context(), aliases(), expr, displayStyle());

  onInit?.(reducer);

  let reduceResults: ReduceResult[] = [];
  while (true) {
    const result = reducer.next();

    if (result.done) break;

    const reduceResult = result.value;

    if (reduceResult == null) continue;

    reduceResults.push(reduceResult);

    if (!reducer.hasNext) break;
    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) break;

    if (reduceResults.length > count * 2) {
      reduceResults = reduceResults.slice(-count);
    }

    await new Promise((resolve) => setTimeout(resolve, 0));
  }

  const tail = reduceResults.slice(-count);
  for (const reduceResult of tail) {
    onReduce?.(reduceResult);
  }

  if (tail.length > 0) {
    onEnd?.(tail[tail.length - 1]);
  }
}

export async function reduceLast(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, onEnd, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), aliases(), expr, displayStyle());

  onInit?.(reducer);

  let reduceResult: ReduceResult | null = null;
  while (true) {
    const result = reducer.next();

    if (result.done) break;

    reduceResult = result.value;

    if (reduceResult == null) continue;

    if (!reducer.hasNext) break;
    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) break;

    await new Promise((resolve) => setTimeout(resolve, 0));
  }

  reduceResult && onReduce?.(reduceResult);
  reduceResult && onEnd?.(reduceResult);
}
