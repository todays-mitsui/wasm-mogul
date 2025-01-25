import { type JSX, For } from "solid-js";
import { context } from "~/signals";
import { type DisplayStyle, renderFunc, sortFuncs } from "~/service/func";
import styles from "./Context.module.css";

interface ContextProps {
  displayStyle?: DisplayStyle;
}

export default function Context(props: ContextProps): JSX.Element {
  const funcs = () =>
    sortFuncs(Object.values(context())).map((func) => renderFunc(func));

  return (
    <div class={styles.context}>
      <h2>Context</h2>
      <ul>
        <For each={funcs()}>
          {([lhs, rhs]) => (
            <li>
              <code>{`${lhs} = ${rhs}`}</code>
            </li>
          )}
        </For>
      </ul>
    </div>
  );
}
