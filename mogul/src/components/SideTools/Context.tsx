import { type JSX, For, Show } from "solid-js";
import { context } from "~/signals";
import { type DisplayStyle, renderFunc, sortFuncs } from "~/service/func";
import { aliases } from "~/service/aliases";
import styles from "./Context.module.css";

interface ContextProps {
  displayStyle?: DisplayStyle;
}

export default function Context(props: ContextProps): JSX.Element {
  const funcs = () =>
    sortFuncs(Object.values(context())).map((func) =>
      renderFunc(func, props.displayStyle),
    );

  return (
    <div class={styles.context}>
      <Show when={aliases().length > 0}>
        <h2>Shortcut Reference</h2>
        <ul>
          <For each={aliases()}>
            {(alias) => (
              <li>
                <code>{alias}</code>
              </li>
            )}
          </For>
        </ul>
      </Show>

      <h2>Context</h2>
      <ul>
        <For each={funcs()}>
          {([signature, body]) => (
            <li>
              <code>{`${signature} = ${body}`}</code>
            </li>
          )}
        </For>
      </ul>
    </div>
  );
}
