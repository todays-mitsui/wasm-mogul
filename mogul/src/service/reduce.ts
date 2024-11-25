import {
  type Expr,
  Reducer,
  type ReduceResult,
  type FormedReducedExpr,
} from "../../../ski3/pkg/index";
import { context, displayStyle } from "~/signals";
export { ReduceResult, FormedReducedExpr };

const MAX_STEPS = 1000;

interface ReduceOptions {
  onInit?: (_: { reducer: Reducer }) => void;
  onReduce?: (_: {
    reducer: Reducer;
    reduceResult: ReduceResult;
  }) => void;
  maxSteps?: number;
}

export async function reduce(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), expr, displayStyle());

  onInit?.({ reducer });

  while (true) {
    const result = reducer.next();

    if (result.done) {
      return;
    }

    const reduceResult = result.value;

    if (reduceResult == null) {
      continue;
    }

    onReduce?.({ reducer, reduceResult });

    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) {
      return;
    }

    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}

export async function reduceLast(
  expr: Expr,
  options?: ReduceOptions,
): Promise<void> {
  const { onInit, onReduce, maxSteps } = options ?? {};

  const reducer = new Reducer(context(), expr, displayStyle());

  onInit?.({ reducer });

  while (true) {
    const result = reducer.next();

    if (result.done) {
      return;
    }

    const reduceResult = result.value;

    if (reduceResult == null) {
      continue;
    }

    if (!reducer.hasNext) {
      onReduce?.({ reducer, reduceResult });
      return;
    }

    if ((maxSteps ?? MAX_STEPS) <= reduceResult.step) {
      return;
    }

    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}
