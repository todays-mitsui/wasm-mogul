import { type JSX, splitProps } from "solid-js";
import styles from "./ToolBox.module.css";
import { type SideTools, sideTools } from "~/signals";
import classNames from "classnames";

interface ToolBoxProps {
  class?: string;
  boxName: SideTools;
  boxTitle: string;
  summary?: string | JSX.Element;
  open: boolean;
  onClick?: () => void;
  children?: JSX.Element;
  [prop: string]: any;
}

export default function ToolBox(props: ToolBoxProps): JSX.Element {
  const [, remainingProps] = splitProps(props, [
    "class",
    "boxName",
    "boxTitle",
    "summary",
    "open",
    "onClick",
    "children",
  ]);

  const onClick: JSX.EventHandler<HTMLElement, MouseEvent> = (event) => {
    event.preventDefault();
    props.onClick?.();
  };

  return (
    <details
      class={classNames(props.class, styles.toolBox)}
      open={props.open}
      {...remainingProps}
    >
      <summary class={styles.summary} title={props.boxTitle} onClick={onClick}>
        {props.summary ?? props.boxName}
      </summary>
      <div class={styles.inner}>{props.children}</div>
    </details>
  );
}
