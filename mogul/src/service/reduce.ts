import { type Expr, Reducer, type ReduceResult } from "../../../ski3/pkg/index";
import { context, displayStyle } from "~/signals";
export { ReduceResult };

const MAX_STEPS = 1000;

export async function reduce<T>(
  expr: Expr,
  onInit: (_: { reducer: Reducer }) => T,
  onReduce: (_: {
    reducer: Reducer;
    reduceResult: ReduceResult;
    payload: T;
  }) => void,
  maxSteps: number = MAX_STEPS,
): Promise<void> {
  console.log({ reduce: expr });

  const reducer = new Reducer(context(), expr, displayStyle());

  const payload = onInit({ reducer });

  while (true) {
    let result = reducer.next();

    if (result.done) {
      break;
    }

    const reduceResult = result.value;

    if (reduceResult == null) {
      continue;
    }

    onReduce({ reducer, reduceResult, payload });

    if (maxSteps <= reduceResult.step) {
      break;
    }

    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}
