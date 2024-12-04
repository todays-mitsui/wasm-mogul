import { context, displayStyle } from "~/signals";
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
  maxSteps?: number;
}

export async function reduceHead(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), expr, displayStyle());

  onInit?.(reducer);

  while (true) {
    const result = reducer.next();

    if (result.done) return;

    const reduceResult = result.value;

    if (reduceResult == null) continue;

    onReduce?.(reduceResult);

    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) return;

    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}

export async function reduceTail(
  expr: Expr,
  options: ReduceOptions & { count: number },
): Promise<void> {
  const { onInit, onReduce, maxSteps, count } = options;

  const reducer = new Reducer(context(), expr, displayStyle());

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
}

export async function reduceLast(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), expr, displayStyle());

  onInit?.(reducer);

  while (true) {
    const result = reducer.next();

    if (result.done) return;

    const reduceResult = result.value;

    if (reduceResult == null) continue;

    if (!reducer.hasNext) {
      onReduce?.(reduceResult);
      return;
    }

    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) return;

    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}
