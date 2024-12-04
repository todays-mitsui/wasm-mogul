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
import { type DisplayStyle, renderFunc } from "~/service/func";
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

function ConsoleUnitReduce(props: ConsoleItemReduce): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{props.formed.expr}</code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Index>
    </ul>
  );
}

function ConsoleUnitReduceLast(props: ConsoleItemReduceLast): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{props.formed.expr}</code>
      </li>
      <Show when={props.reduceResult()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Show>
    </ul>
  );
}

function ConsoleUnitReduceHead(props: ConsoleItemReduceHead): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{props.formed.expr}</code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
          </li>
        )}
      </Index>
    </ul>
  );
}

function ConsoleUnitReduceTail(props: ConsoleItemReduceTail): JSX.Element {
  return (
    <ul class={classNames(styles.unit, styles.ordered, styles.reduce)}>
      <li data-step="0">
        <code>{props.formed.expr}</code>
      </li>
      <Index each={props.reduceResults()}>
        {(result) => (
          <li data-step={result().step}>
            <code>{result().formed.expr}</code>
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