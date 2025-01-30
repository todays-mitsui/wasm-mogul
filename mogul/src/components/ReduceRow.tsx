import { type JSX, onMount } from "solid-js";
import {
  type ExprRange,
  type ReducibleRange,
  markReduced,
  markReducible,
} from "~/lib/mark";

interface ReduceRowProps {
  expr: string;
  reducedRange?: ExprRange | null;
  reducibleRange?: ReducibleRange | null;
}

export function ReduceRow(props: ReduceRowProps): JSX.Element {
  let reducedRef: HTMLSpanElement | undefined;
  let reducibleRef: HTMLSpanElement | undefined;

  onMount(() => {
    if (props.reducedRange == null && props.reducibleRange == null) {
      if (reducibleRef != null) {
        reducibleRef.innerText = props.expr;
      }
    }

    if (props.reducedRange != null) {
      const reduced = markReduced(props.expr, props.reducedRange);
      reducedRef?.appendChild(reduced);
    }

    if (props.reducibleRange != null) {
      const reducible = markReducible(props.expr, props.reducibleRange);
      reducibleRef?.appendChild(reducible);
    }
  });

  return (
    <>
      <span class="expr-mark-reduced" ref={reducedRef} />
      <span class="expr-mark-reducible" ref={reducibleRef} />
    </>
  );
}
