import {
  type ConsoleItem,
  type ConsoleItemUpdate,
  type ConsoleItemDelete,
  type ConsoleItemQueryDefined,
  type ConsoleItemQueryUndefined,
  type ConsoleItemContext,
  console,
  context,
} from "~/signals";
import { renderFunc } from "~/service/func";
import { type JSX, Index, For } from "solid-js";

export default function Console() {
  return (
    <div>
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
    <ul>
      <li>{signature}</li>
      <li>{body}</li>
    </ul>
  );
}

function ConsoleUnitDelete(item: ConsoleItemDelete): JSX.Element {
  return (
    <ul>
      <li>{item.identifier}</li>
      <li>{item.identifier}</li>
    </ul>
  );
}

function ConsoleUnitQueryDefined(item: ConsoleItemQueryDefined): JSX.Element {
  const [signature, body] = renderFunc(item.func);
  return (
    <ul>
      <li>{signature}</li>
      <li>{body}</li>
    </ul>
  );
}

function ConsoleUnitQueryUndefined(
  item: ConsoleItemQueryUndefined,
): JSX.Element {
  return (
    <ul>
      <li>{item.identifier}</li>
      <li>{item.identifier}</li>
    </ul>
  );
}

function ConsoleUnitContext(item: ConsoleItemContext): JSX.Element {
  const functions = Object.values(context()).toSorted((a, b) =>
    a.name < b.name ? -1 : 1,
  );
  return (
    <ul>
      <For each={functions}>
        {(func) => {
          const [signature, body] = renderFunc(func);
          return <li>{`${signature} = ${body}`}</li>;
        }}
      </For>
    </ul>
  );
}
