import { type JSX, For } from "solid-js";
import { context } from "~/signals";
import { type DisplayStyle, renderFunc } from "~/service/func";
import styles from "./Context.module.css";

interface ContextProps {
  displayStyle?: DisplayStyle;
}

export default function Context(props: ContextProps): JSX.Element {
  const funcs = () =>
    Object.entries(context()).map(([key, func]) => [key, renderFunc(func)]);

  return (
    <div class={styles.context}>
      <h2>Context</h2>
      <ul>
        <For each={funcs()}>
          {([, func]) => (
            <li>
              <code>{`${func[0]} = ${func[1]}`}</code>
            </li>
          )}
        </For>
      </ul>
    </div>
  );
}
