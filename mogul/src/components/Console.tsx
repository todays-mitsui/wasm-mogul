import {
  type ConsoleItem,
  type ConsoleItemUpdate,
  type ConsoleItemDelete,
  type ConsoleItemReduce,
  type ConsoleItemReduceLast,
  type ConsoleItemReduceHead,
  type ConsoleItemReduceTail,
  type ConsoleItemQueryDefined,
  type ConsoleItemQueryUndefined,
  type ConsoleItemContext,
  console,
  context,
} from "~/signals";
import { renderFunc } from "~/service/func";
import { type JSX, Index, For, Show } from "solid-js";
import styles from "./Console.module.css";
import classNames from "classnames";

interface Props {
  class?: string | string[];
}

export default function Console(props: Props): JSX.Element {
  const className =
    props.class == null
      ? []
      : Array.isArray(props.class)
        ? props.class
        : [props.class];

  return (
    <div class={classNames(...className, styles.console)}>
      <Index each={console()} fallback={<div>Loading...</div>}>
        {(item) => <ConsoleUnit {...item()} />}
      </Index>
    </div>
  );
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
    default:
      return null;
  }
}

function ConsoleUnitUpdate(item: ConsoleItemUpdate): JSX.Element {
  const [signature, body] = renderFunc(item.func);
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

function ConsoleUnitDelete(item: ConsoleItemDelete): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{item.identifier}</code>
      </li>
      <li>
        <code>{item.identifier}</code>
      </li>
    </ul>
  );
}

function ConsoleUnitReduce(item: ConsoleItemReduce): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{item.formed.expr}</code>
      </li>
      <Index each={item.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Index>
    </ul>
  );
}

function ConsoleUnitReduceLast(item: ConsoleItemReduceLast): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{item.formed.expr}</code>
      </li>
      <Show when={item.reduceResult()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Show>
    </ul>
  );
}

function ConsoleUnitReduceHead(item: ConsoleItemReduceHead): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{item.formed.expr}</code>
      </li>
      <Index each={item.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Index>
    </ul>
  );
}

function ConsoleUnitReduceTail(item: ConsoleItemReduceTail): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{item.formed.expr}</code>
      </li>
      <Index each={item.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Index>
    </ul>
  );
}

function ConsoleUnitQueryDefined(item: ConsoleItemQueryDefined): JSX.Element {
  const [signature, body] = renderFunc(item.func);
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

function ConsoleUnitQueryUndefined(
  item: ConsoleItemQueryUndefined,
): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.unordered, styles.define)}>
      <li>
        <code>{item.identifier}</code>
      </li>
      <li>
        <code>{item.identifier}</code>
      </li>
    </ul>
  );
}

function ConsoleUnitContext(item: ConsoleItemContext): JSX.Element {
  const functions = Object.values(context()).toSorted((a, b) =>
    a.name < b.name ? -1 : 1,
  );
  return (
    <ul class={classNames(styles.unit, styles.unordered)}>
      <For each={functions}>
        {(func) => {
          const [signature, body] = renderFunc(func);
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
