import { type JSX, children, splitProps } from "solid-js";
import styles from "./ToolBox.module.css";

interface Props {
  boxName: string;
  boxTitle: string;
  children?: JSX.Element;
  [prop: string]: any;
}

export default function ToolBox(props: Props): JSX.Element {
  const [, remainingProps] = splitProps(props, ["name", "title", "children"]);
  return (
    <details class={styles.toolBox} {...remainingProps}>
      <summary class={styles.summary} title={props.boxTitle}>
        {props.boxName}
      </summary>
      <div class={styles.inner}>{props.children}</div>
    </details>
  );
}
