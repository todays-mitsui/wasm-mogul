import classNames from "classnames";
import { For, Index, type JSX, Show, createEffect, onCleanup } from "solid-js";
import { type DisplayStyle, renderFunc, sortFuncs } from "~/service/func";
import {
  consoleOut,
  type ConsoleItem,
  type ConsoleItemContext,
  type ConsoleItemDelete,
  type ConsoleItemQueryDefined,
  type ConsoleItemQueryUndefined,
  type ConsoleItemReduce,
  type ConsoleItemReduceHead,
  type ConsoleItemReduceLast,
  type ConsoleItemReduceTail,
  type ConsoleItemUnlambda,
  type ConsoleItemUpdate,
  context,
  sideTools,
} from "~/signals";
import styles from "./Console.module.css";
import { ReduceRow } from "./ReduceRow";
import { renderExpr } from "~/service/unlambda";

interface Props {
  class?: string;
}

export default function Console(props: Props): JSX.Element {
  let wrapper: HTMLDivElement | undefined;
  createEffect(() => {
    if (wrapper == null) return;
    const observer = createObserver(wrapper);
    onCleanup(() => {
      observer.disconnect();
    });
  });

  return (
    <div
      class={classNames(props.class, styles.console)}
      onClick={() => sideTools.closeAll()}
      ref={wrapper}
    >
      <Index each={consoleOut()} fallback={<div>Loading...</div>}>
        {(item) => <ConsoleUnit {...item()} />}
      </Index>
    </div>
  );
}

function createObserver(wrapper: HTMLDivElement): MutationObserver {
  const observer = new MutationObserver((mutationsList) => {
    for (const mutation of mutationsList) {
      if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
        setTimeout(() => {
          wrapper.scrollTo({
            top: wrapper.scrollHeight,
            behavior: "smooth",
          });
        }, 100);
      }
    }
  });
  observer.observe(wrapper, {
    childList: true, // 子要素の追加や削除を監視
    subtree: false, // 子孫要素は監視しない
  });
  return observer;
}

// ========================================================================== //

function ConsoleUnit(item: ConsoleItem): JSX.Element {
  switch (item.type) {
    case "Update":
      return <ConsoleUnitUpdate {...item} />;
    case "Delete":
      return <ConsoleUnitDelete {...item} />;
    case "Reduce":
      return <ConsoleUnitReduce {...item} />;
    case "ReduceLast":
      return <ConsoleUnitReduceLast {...item} />;
    case "ReduceHead":
      return <ConsoleUnitReduceHead {...item} />;
    case "ReduceTail":
      return <ConsoleUnitReduceTail {...item} />;
    case "QueryDefined":
      return <ConsoleUnitQueryDefined {...item} />;
    case "QueryUndefined":
      return <ConsoleUnitQueryUndefined {...item} />;
    case "Context":
      return <ConsoleUnitContext {...item} />;
    case "Unlambda":
      return <ConsoleUnitUnlambda {...item} />;
    default:
      return null;
  }
}

export function ConsoleUnitUpdate(
  props: ConsoleItemUpdate & { displayStyle?: DisplayStyle },
): JSX.Element {
  const [signature, body] = renderFunc(props.func, props.displayStyle);
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{signature}</code>
      </li>
      <li>
        <code>{body}</code>
      </li>
    </ul>
  );
}

function ConsoleUnitDelete(props: ConsoleItemDelete): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{props.identifier}</code>
      </li>
      <li>
        <code>{props.identifier}</code>
      </li>
    </ul>
  );
}

export function ConsoleUnitReduce(props: ConsoleItemReduce): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>
          <ReduceRow
            expr={props.formed.expr}
            reducibleRange={props.formed.reducibleRange}
          />
        </code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>
              <ReduceRow
                expr={result().formed.expr}
                reducedRange={result().formed.reducedRange}
                reducibleRange={result().formed.reducibleRange}
              />
            </code>
          </li>
        )}
      </Index>
    </ul>
  );
}

export function ConsoleUnitReduceLast(
  props: ConsoleItemReduceLast,
): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduceLast)}>
      <li data-step="0">
        <code>
          <ReduceRow
            expr={props.formed.expr}
            reducibleRange={props.formed.reducibleRange}
          />
        </code>
      </li>
      <li class={styles.omitted}>
        <code>……</code>
      </li>
      <Show when={props.reduceResult()}>
        {(result) => (
          <li data-step={result().step}>
            <code>
              <ReduceRow
                expr={result().formed.expr}
                reducedRange={result().formed.reducedRange}
                reducibleRange={result().formed.reducibleRange}
              />
            </code>
          </li>
        )}
      </Show>
    </ul>
  );
}

export function ConsoleUnitReduceHead(
  props: ConsoleItemReduceHead,
): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduceHead)}>
      <li data-step="0">
        <code>
          <ReduceRow
            expr={props.formed.expr}
            reducibleRange={props.formed.reducibleRange}
          />
        </code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>
              <ReduceRow
                expr={result().formed.expr}
                reducedRange={result().formed.reducedRange}
                reducibleRange={result().formed.reducibleRange}
              />
            </code>
          </li>
        )}
      </Index>
    </ul>
  );
}

export function ConsoleUnitReduceTail(
  props: ConsoleItemReduceTail,
): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduceTail)}>
      <li data-step="0">
        <code>
          <ReduceRow
            expr={props.formed.expr}
            reducibleRange={props.formed.reducibleRange}
          />
        </code>
      </li>
      <li class={styles.omitted}>
        <code>……</code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>
              <ReduceRow
                expr={result().formed.expr}
                reducedRange={result().formed.reducedRange}
                reducibleRange={result().formed.reducibleRange}
              />
            </code>
          </li>
        )}
      </Index>
    </ul>
  );
}

export function ConsoleUnitQueryDefined(
  props: ConsoleItemQueryDefined & { displayStyle?: DisplayStyle },
): JSX.Element {
  const [signature, body] = renderFunc(props.func, props.displayStyle);
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{signature}</code>
      </li>
      <li>
        <code>{body}</code>
      </li>
    </ul>
  );
}

export function ConsoleUnitQueryUndefined(
  props: ConsoleItemQueryUndefined,
): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{props.identifier}</code>
      </li>
      <li>
        <code>{props.identifier}</code>
      </li>
    </ul>
  );
}

function ConsoleUnitContext(_props: ConsoleItemContext): JSX.Element {
  const funcs = sortFuncs(Object.values(context())).map((func) =>
    renderFunc(func),
  );

  return (
    <ul class={classNames(styles.unit, styles.unordered)}>
      <For each={funcs}>
        {([signature, body]) => {
          return (
            <li>
              <code>{`${signature} = ${body}`}</code>
            </li>
          );
        }}
      </For>
    </ul>
  );
}

function ConsoleUnitUnlambda(
  props: ConsoleItemUnlambda & { displayStyle?: DisplayStyle },
): JSX.Element {
  const expr = renderExpr(props.expr, props.displayStyle);
  const result = renderExpr(props.result, props.displayStyle);

  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.unlambda)}>
      <li>
        <code>{expr}</code>
      </li>
      <li>
        <code>{result}</code>
      </li>
    </ul>
  );
}
