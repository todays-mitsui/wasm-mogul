import { For, Index, type JSX, Show, onMount } from "solid-js";
import {
  type ExprRange,
  type ReducibleRange,
  markReduced,
  markReducible,
} from "~/lib/mark";

export function ReduceRow(props: {
  expr: string;
  reducedRange?: ExprRange | null;
  reducibleRange?: ReducibleRange | null;
}): JSX.Element {
  let reducedRef: HTMLSpanElement | undefined;
  if (props.reducedRange != null) {
    const reduced = markReduced(props.expr, props.reducedRange);
    onMount(() => {
      reducedRef?.appendChild(reduced);
    });
  }

  let reducibleRef: HTMLSpanElement | undefined;
  if (props.reducibleRange != null) {
    const reducible = markReducible(props.expr, props.reducibleRange);
    onMount(() => {
      reducibleRef?.appendChild(reducible);
    });
  }

  return (
    <>
      <span class="expr-mark-reduced" ref={reducedRef} />
      <span class="expr-mark-reducible" ref={reducibleRef} />
    </>
  );
}
